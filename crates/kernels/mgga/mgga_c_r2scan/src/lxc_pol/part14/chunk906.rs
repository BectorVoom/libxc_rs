//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 906/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk906<F: Float>(t1216: F, t313: F, t6678: F, t806: F, t810: F, t8316: F, t8323: F, t8326: F, t8329: F, t8337: F, t8344: F, t8347: F, t8350: F, t8377: F, t8385: F) -> F {
    let t8395 = F::new(3.0) / F::new(10.0) * t313 * (-F::new(10.0) / F::new(27.0) * t8316 + F::new(20.0) / F::new(9.0) * t8377 * t1216 * t806 + F::new(10.0) / F::new(9.0) * t8323 + F::new(5.0) / F::new(3.0) * t8326 - F::new(5.0) * t8329 - F::new(10.0) / F::new(27.0) * t8337 - F::new(20.0) / F::new(9.0) * t8385 * t1216 * t810 + F::new(10.0) / F::new(9.0) * t8344 - F::new(5.0) / F::new(3.0) * t8347 + F::new(5.0) * t8350) - t6678;
    t8395
}
