//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1125/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1125(t3678: f64, t5327: f64, t5323: f64, t3667: f64, t5362: f64, t1789: f64, t371: f64, t676: f64, t1235: f64, t1769: f64, t3565: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17296 = 0.28582678745379824648e-3_f64 * t5327 * t3678;
    let t17298 = 0.15244095330869239812e-2_f64 * t5323 * t3678;
    let t17301 = 0.28582678745379824648e-3_f64 * t3667 * t5362;
    let t17303 = t371 * t676 * t1789;
    let t17304 = t1235 * t17303;
    let t17306 = t1769 * t3565;
    let t17307 = t17306 * t225;
    (t17296, t17298, t17301, t17304, t17306, t17307)
}
