//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3007/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3007(t20020: f64, t4858: f64, t1011: f64, t140: f64, t23877: f64, t1043: f64, t24031: f64, t15823: f64, t20029: f64, t1045: f64, t15696: f64, t15700: f64, t19625: f64, t19981: f64, t23878: f64, t24024: f64, t3117: f64, t3181: f64, t3211: f64, t3241: f64, t372: f64, t42328: f64, t43069: f64, t43291: f64, t4782: f64, t55034: f64, t6299: f64, t66306: f64, t67152: f64, t67186: f64, t67195: f64, t67199: f64, t67206: f64, t67213: f64, t67237: f64, t67249: f64, t67253: f64) -> (f64, f64) {
    let t79874 = t4858 * t20020;
    let t79881 = t1011 * t140 * t23877;
    let t79884 = t24031 * t1043;
    let t79892 = t15823 * t20029;
    let t79907 = 0.42874018118069736972e-3_f64 * t42328 * t15696 * t19625 - 0.42874018118069736972e-3_f64 * t79874 + 0.11433071498151929859e-2_f64 * t3211 * t24024 + 2.0_f64 / 27.0_f64 * t3241 * t23878 - t79881 / 108.0_f64 + 0.42874018118069736972e-3_f64 * t67152 - 0.12862205435420921092e-2_f64 * t43291 * t3117 * t79884 * t1045 + 0.14291339372689912324e-3_f64 * t67186 + 0.28582678745379824648e-3_f64 * t67195 + 0.85748036236139473944e-3_f64 * t67199 + 0.85748036236139473947e-3_f64 * t79892 - 0.28582678745379824648e-3_f64 * t67206 - 0.85748036236139473944e-3_f64 * t67213 + 0.85748036236139473944e-3_f64 * t67237 + 0.7145669686344956162e-3_f64 * t15700 * t372 * t3181 * t6299 * t19981 + 0.85748036236139473947e-3_f64 * t43069 * t66306 * t4782 + 0.28582678745379824648e-3_f64 * t67249 + t55034 + 0.17149607247227894789e-2_f64 * t67253;
    (t79884, t79907)
}
