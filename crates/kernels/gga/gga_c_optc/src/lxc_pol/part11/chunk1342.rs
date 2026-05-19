//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1342/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1342<F: Float>(t5059: F, t24357: F, t277: F, t33574: F, t33596: F, t52260: F, t52264: F, t57251: F, t57253: F, t57257: F, t57260: F, t57520: F, t57523: F, t57525: F, t95: F) -> F {
    let t58229 = t5059 * t5059;
    let t58237 = F::new(20.0) / F::new(81.0) * t33574 - t57251 + t57253 + t57257 - F::cast_from(0.15506928860942058298e-1_f64) * t95 * t277 * t58229 * t24357 + t57260 + F::new(20.0) / F::new(27.0) * t33596 + t57520 - t57523 - t57525 + F::new(56.0) / F::new(81.0) * t52260 + F::new(8.0) / F::new(9.0) * t52264;
    t58237
}
