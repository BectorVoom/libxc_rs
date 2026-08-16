//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1681/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1681(t15191: f64, t4628: f64, t698: f64, t15127: f64, t15125: f64, t11452: f64, t1621: f64, t3014: f64, t4707: f64, t11509: f64, t1633: f64, t15168: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15192 = 0.20128333333333333334e0_f64 * t15191;
    let t15197 = t698 * t4628;
    let t15198 = 0.11038e0_f64 * t15197;
    let t15209 = 4.0_f64 / 27.0_f64 * t15127;
    let t15210 = 4.0_f64 / 9.0_f64 * t15125;
    let t15211 = 2.0_f64 / 9.0_f64 * t15191;
    let t15241 = t1621 * t11452;
    let t15258 = t4707 * t3014;
    let t15266 = t1633 * t11509;
    let t15301 = 0.22954444444444444444e0_f64 * t15127;
    let t15312 = 0.27785333333333333334e0_f64 * t15168;
    (t15192, t15197, t15198, t15209, t15210, t15211, t15241, t15258, t15266, t15301, t15312)
}
