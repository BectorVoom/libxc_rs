//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 946/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk946(t125: f64, t1868: f64, t1399: f64, t3936: f64, t1370: f64, t3934: f64, t3944: f64, t3950: f64, t3953: f64, t3958: f64, t3967: f64, t3976: f64, t3982: f64, t3987: f64, t3990: f64, t3996: f64, t5681: f64, t5686: f64, t5690: f64, t5697: f64, t5701: f64) -> (f64, f64, f64) {
    let t5704 = t125 * t1868;
    let t5705 = t5704 * t1399;
    let t5706 = t3936 * t5705;
    let t5709 = 7.0_f64 / 144.0_f64 * t5681 + 0.28582678745379824648e-4_f64 * t3953 - t3976 + t3987 + 7.0_f64 / 144.0_f64 * t3958 + t3944 * t5686 / 16.0_f64 + t3967 - t1370 * t5690 / 48.0_f64 - 0.50820002809285328224e-4_f64 * t3982 + 0.40015750243531754508e-2_f64 * t3990 + 0.71456696863449561619e-5_f64 * t3996 + 0.85748036236139473944e-3_f64 * t3934 * t5697 - 0.21437009059034868486e-3_f64 * t3934 * t5701 + 0.85748036236139473944e-3_f64 * t3934 * t5706 + t3950;
    (t5704, t5706, t5709)
}
