//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3182/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3182(t5245: f64, t6587: f64, t20820: f64, t5265: f64, t20851: f64, t5362: f64, t1042: f64, t1121: f64, t1261: f64, t17505: f64, t17654: f64, t1794: f64, t1808: f64, t20907: f64, t20914: f64, t21272: f64, t247: f64, t3604: f64, t3719: f64, t5384: f64, t5391: f64, t5397: f64, t58983: f64, t606: f64, t69723: f64, t70756: f64, t70758: f64, t71300: f64, t78785: f64) -> (f64, f64) {
    let t83567 = t5245 * t6587;
    let t83580 = t20820 * t5265;
    let t83584 = t20851 * t5362;
    let t83592 = -0.14481890564325777821e-1_f64 * t69723 * t1808 - 0.14481890564325777821e-1_f64 * t21272 * t5397 + 0.12862205435420921092e-2_f64 * t5384 * t247 * t3719 * t83567 - 0.14481890564325777821e-1_f64 * t70756 + 0.28582678745379824648e-3_f64 * t70758 + 0.45732285992607719436e-2_f64 * t5391 * t20907 + 0.23289590088828005269e-2_f64 * t1261 * t1042 * t58983 * t78785 + 0.42874018118069736972e-3_f64 * t83580 - 0.45732285992607719436e-2_f64 * t17505 * t20914 - 0.42874018118069736972e-3_f64 * t83584 - 0.85748036236139473947e-3_f64 * t17654 * t71300 * t3604 * t1794 * t1121 * t606;
    (t83567, t83592)
}
