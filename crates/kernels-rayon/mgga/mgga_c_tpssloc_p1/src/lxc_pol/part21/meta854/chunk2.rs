//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3089/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3089(t1107: f64, t63996: f64, t1102: f64, t4756: f64, t14804: f64, t14801: f64, t3270: f64, t64008: f64, t1113: f64, t136: f64, t63353: f64, t43780: f64, t43782: f64, t43816: f64, t44053: f64, t50968: f64, t50970: f64, t50972: f64, t50978: f64, t51039: f64, t51041: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t64028 = t1107 * t63996;
    let t64030 = t1102 * t4756;
    let t64031 = t14804 * t64030;
    let t64033 = t14801 * t64030;
    let t64042 = t3270 * t64008;
    let t64045 = t136 * t1113 * t63353;
    let t64049 = 0.3071625e0_f64 * t64028 - 0.3071625e0_f64 * t64031 + 0.5696775e1_f64 * t64033 + 0.73028148148148148146e-1_f64 * t50968 + 0.36514074074074074073e-1_f64 * t50970 + 0.21908444444444444444e0_f64 * t50972 + t44053 - 0.48685432098765432097e-1_f64 * t50978 + 0.13287407407407407408e0_f64 * t43780 + 0.26574814814814814816e0_f64 * t43782 - 0.62007901234567901237e0_f64 * t43816 - 0.1898925e1_f64 * t64042 + 0.82156666666666666667e-1_f64 * t64045 + 0.73028148148148148147e0_f64 * t51039 - 0.21908444444444444444e0_f64 * t51041;
    (t64028, t64031, t64033, t64042, t64045, t64049)
}
