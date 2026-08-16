//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3149/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3149(t17351: f64, t17354: f64, t56756: f64, t3588: f64, t3611: f64, t12904: f64, t5293: f64, t12959: f64, t17569: f64, t11262: f64, t1261: f64, t5269: f64) -> (f64, f64, f64, f64, f64) {
    let t56758 = t17351 * t56756 * t17354;
    let t56760 = t3611 * t3588;
    let t56785 = t5293 * t12904;
    let t56787 = t17569 * t12959;
    let t56790 = t1261 * t11262 * t5269;
    (t56758, t56760, t56785, t56787, t56790)
}
