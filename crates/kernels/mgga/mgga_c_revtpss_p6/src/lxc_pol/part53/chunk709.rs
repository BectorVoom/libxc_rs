//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 709/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk709<F: Float>(t572: F, t7334: F, t1461: F, t2040: F, t573: F, t7324: F, t7329: F, t7333: F, t2121: F, t38: F) -> (F, F, F) {
    let t7336 = F::cast_from(3.0_f64) * t572 * t7334;
    let t7337 = F::cast_from(3.0_f64) * t1461 * t2040 + t573 * t7324 + t7329 + t7333 + t7336;
    let t7565 = t38 * t2121;
    (t7336, t7337, t7565)
}
