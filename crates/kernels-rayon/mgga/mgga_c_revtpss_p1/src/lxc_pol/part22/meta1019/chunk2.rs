//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3533/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3533(t11933: f64, t19976: f64, t3115: f64, t42793: f64, t6272: f64, t11922: f64, t16081: f64, t19749: f64, t11866: f64, t15758: f64, t15917: f64, t15922: f64, t15926: f64, t16052: f64, t16078: f64, t19726: f64, t19758: f64, t20066: f64, t20070: f64, t20075: f64, t20105: f64, t42643: f64, t42830: f64, t4891: f64, t4896: f64, t4907: f64, t53855: f64, t55958: f64) -> f64 {
    let t67006 = t11933 * t19976;
    let t67015 = t3115 * t42793 * t6272;
    let t67025 = t16081 * t11922 * t19749;
    let t67031 = -0.30488190661738479624e-2_f64 * t16052 * t19726 - 0.85748036236139473944e-3_f64 * t42643 * t20075 + 0.30488190661738479624e-2_f64 * t67006 - 0.85748036236139473944e-3_f64 * t53855 * t4907 - 0.85748036236139473944e-3_f64 * t15926 * t15922 - 0.42874018118069736972e-3_f64 * t15926 * t16078 + 0.95275595817932748827e-4_f64 * t67015 + 0.85748036236139473944e-3_f64 * t15758 * t20066 - 0.42874018118069736972e-3_f64 * t15917 * t20070 + 0.17149607247227894789e-2_f64 * t55958 * t4891 * t4896 + 0.17149607247227894789e-2_f64 * t67025 + 0.42874018118069736972e-3_f64 * t42830 * t19758 - 0.42874018118069736972e-3_f64 * t11866 * t20105;
    t67031
}
