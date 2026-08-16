//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3017/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3017(t3140: f64, t4743: f64, t3149: f64, t3160: f64, t15690: f64, t3153: f64, t372: f64, t11921: f64, t15716: f64, t15717: f64, t247: f64, t1043: f64, t11804: f64, t11871: f64, t15758: f64, t15782: f64, t15817: f64, t1592: f64, t15926: f64, t16089: f64, t16102: f64, t16103: f64, t16205: f64, t3092: f64, t3136: f64, t3157: f64, t3164: f64, t3188: f64, t42328: f64, t43069: f64, t43082: f64, t4823: f64, t4894: f64, t4900: f64) -> f64 {
    let t55201 = t4743 * t3140;
    let t55202 = t55201 * t3149;
    let t55205 = t55201 * t3160;
    let t55209 = t372 * t15690 * t3153;
    let t55233 = t15716 * t247 * t11921 * t15717;
    let t55237 = 0.64311027177104605458e-3_f64 * t15817 * t3136 + 0.12862205435420921092e-2_f64 * t55202 * t3157 - 0.64311027177104605458e-3_f64 * t55205 * t3164 - 0.17149607247227894789e-2_f64 * t43082 * t55209 * t4894 * t16102 + 0.17149607247227894789e-2_f64 * t43069 * t372 * t4823 * t1043 * t16103 + 0.85748036236139473944e-3_f64 * t42328 * t55209 * t4900 * t16102 + 0.85748036236139473944e-3_f64 * t16089 * t3092 * t1592 * t11804 - 0.64311027177104605458e-3_f64 * t15926 * t11871 + 0.25724410870841842183e-2_f64 * t15758 * t15782 - 0.25724410870841842184e-2_f64 * t55233 + 0.71456696863449561621e-3_f64 * t3188 * t16205;
    t55237
}
