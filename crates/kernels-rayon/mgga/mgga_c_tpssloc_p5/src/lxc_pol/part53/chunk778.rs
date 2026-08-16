//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 778/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk778(t192: f64, t531: f64, t1982: f64, t25: f64, t870: f64, t4255: f64, t16596: f64, t22960: f64, t1484: f64, t606: f64, t4119: f64, t7484: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24994 = t192 * t531;
    let t24995 = t1982 * t24994;
    let t25014 = t870 * t25;
    let t25015 = t25014 * t4255;
    let t25021 = t22960 * t16596;
    let t25024 = t606 * t1484;
    let t25028 = t25 * t4119;
    let t25035 = t794 * t7484;
    (t24994, t24995, t25015, t25021, t25024, t25028, t25035)
}
