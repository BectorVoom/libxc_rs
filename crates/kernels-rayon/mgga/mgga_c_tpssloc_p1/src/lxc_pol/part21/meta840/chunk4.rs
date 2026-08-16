//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3017/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3017(t3185: f64, t61734: f64, t1063: f64, t11037: f64, t14572: f64, t14618: f64, t14622: f64, t14631: f64, t14654: f64, t17671: f64, t17876: f64, t18081: f64, t18108: f64, t18150: f64, t3076: f64, t3189: f64, t3200: f64, t3204: f64, t384: f64, t4615: f64, t4649: f64, t4669: f64, t4684: f64, t4691: f64, t47853: f64, t50508: f64, t50509: f64, t5903: f64, t5936: f64, t5941: f64, t62604: f64) -> f64 {
    let t63183 = t61734 * t3185;
    let t63198 = 24.0_f64 * t17671 * t4649 * t50508 * t50509 - t14622 * t3200 * t5936 - 4.0_f64 * t18150 * t3200 * t4684 + 2.0_f64 * t1063 * t17876 - 2.0_f64 * t11037 * t18081 - 4.0_f64 * t11037 * t18108 + 2.0_f64 * t14572 * t4669 + 4.0_f64 * t14618 * t14654 + 2.0_f64 * t14631 * t47853 + t3076 * t5941 + 2.0_f64 * t3189 * t63183 + t3204 * t5903 + t384 * t62604 + 4.0_f64 * t4615 * t4691;
    t63198
}
