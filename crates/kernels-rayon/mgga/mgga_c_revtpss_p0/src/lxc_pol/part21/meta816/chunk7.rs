//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3002/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3002(t12047: f64, t53740: f64, t16138: f64, t372: f64, t16158: f64, t3106: f64, t12003: f64, t1659: f64, t11648: f64, t4879: f64, t1042: f64, t1068: f64, t11286: f64, t11679: f64, t11705: f64, t11774: f64, t11983: f64, t11994: f64, t12131: f64, t15689: f64, t15691: f64, t15696: f64, t15697: f64, t15707: f64, t16140: f64, t19980: f64, t3075: f64, t3095: f64, t3096: f64, t3127: f64, t4186: f64, t42155: f64, t42804: f64, t4834: f64, t4872: f64) -> f64 {
    let t54811 = t12047 * t53740;
    let t54818 = t372 * t16138;
    let t54836 = t3106 * t16158;
    let t54838 = t1659 * t12003;
    let t54841 = t4879 * t11648;
    let t54843 = -0.7145669686344956162e-3_f64 * t15689 * t19980 * t12131 * t11705 + 0.42874018118069736972e-3_f64 * t54811 * t15691 * t42804 * t3095 - 0.85748036236139473944e-3_f64 * t42155 * t15697 - 0.85748036236139473944e-3_f64 * t11774 * t54818 * t3096 - 0.42874018118069736972e-3_f64 * t11774 * t15696 * t11679 - 0.7145669686344956162e-3_f64 * t15707 * t11286 - 0.42874018118069736972e-3_f64 * t3127 * t1042 * t4872 * t4186 * t3075 - 0.85748036236139473944e-3_f64 * t11994 * t16140 + 0.71456696863449561621e-3_f64 * t4834 * t11983 - 0.30488190661738479624e-2_f64 * t54836 + 0.14481890564325777821e-1_f64 * t54838 * t1068 + 0.42874018118069736972e-3_f64 * t54841;
    t54843
}
