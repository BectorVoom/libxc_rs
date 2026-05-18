//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 756/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk756<F: Float>(t10231: F, t10239: F, t10245: F, t12324: F, t12381: F, t145: F, t169: F, t242: F, t5700: F, t5707: F, t5717: F, t5730: F, t5732: F, t8347: F, t8357: F, t8363: F, t8373: F) -> F {
    let t12384 = t5700 - F::new(0.42447554366239164361e0) * t8363 - t5707 + F::new(0.15917832887339686635e0) * t10231 + F::new(0.3183566577467937327e0) * t8357 + t5717 - F::new(0.31835665774679373271e-1) * t169 * t12324 * t242 - F::new(0.95506997324038119813e-1) * t10239 - F::new(0.95506997324038119813e-1) * t8373 - t5730 - t5732 + F::new(0.9598512193592288454e0) * t8347 - F::new(0.3199504064530762818e0) * t10245 + F::new(0.533250677421793803e-1) * t145 * t12381;
    t12384
}
