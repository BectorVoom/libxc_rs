//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2196/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2196<F: Float>(t21804: F, t76: F, t2242: F, t5819: F, t38: F, t60670: F, t13272: F, t1470: F, t29543: F, t644: F, t77: F, t1497: F, t7719: F) -> (F, F, F, F, F, F) {
    let t108941 = t76 * t21804;
    let t108945 = t2242 * t5819;
    let t108952 = t60670 * t38;
    let t108966 = t13272 * t1470;
    let t108975 = t77 * t29543 * t644;
    let t108978 = t7719 * t1497;
    (t108941, t108945, t108952, t108966, t108975, t108978)
}
