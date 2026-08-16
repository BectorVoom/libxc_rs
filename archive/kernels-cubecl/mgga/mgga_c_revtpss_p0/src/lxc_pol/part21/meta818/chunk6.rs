//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3017/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3017<F: Float>(t3140: F, t4743: F, t3149: F, t3160: F, t15690: F, t3153: F, t372: F, t11921: F, t15716: F, t15717: F, t247: F, t1043: F, t11804: F, t11871: F, t15758: F, t15782: F, t15817: F, t1592: F, t15926: F, t16089: F, t16102: F, t16103: F, t16205: F, t3092: F, t3136: F, t3157: F, t3164: F, t3188: F, t42328: F, t43069: F, t43082: F, t4823: F, t4894: F, t4900: F) -> F {
    let t55201 = t4743 * t3140;
    let t55202 = t55201 * t3149;
    let t55205 = t55201 * t3160;
    let t55209 = t372 * t15690 * t3153;
    let t55233 = t15716 * t247 * t11921 * t15717;
    let t55237 = F::cast_from(0.64311027177104605458e-3_f64) * t15817 * t3136 + F::cast_from(0.12862205435420921092e-2_f64) * t55202 * t3157 - F::cast_from(0.64311027177104605458e-3_f64) * t55205 * t3164 - F::cast_from(0.17149607247227894789e-2_f64) * t43082 * t55209 * t4894 * t16102 + F::cast_from(0.17149607247227894789e-2_f64) * t43069 * t372 * t4823 * t1043 * t16103 + F::cast_from(0.85748036236139473944e-3_f64) * t42328 * t55209 * t4900 * t16102 + F::cast_from(0.85748036236139473944e-3_f64) * t16089 * t3092 * t1592 * t11804 - F::cast_from(0.64311027177104605458e-3_f64) * t15926 * t11871 + F::cast_from(0.25724410870841842183e-2_f64) * t15758 * t15782 - F::cast_from(0.25724410870841842184e-2_f64) * t55233 + F::cast_from(0.71456696863449561621e-3_f64) * t3188 * t16205;
    t55237
}
