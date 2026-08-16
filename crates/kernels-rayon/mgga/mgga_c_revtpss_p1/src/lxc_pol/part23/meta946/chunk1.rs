//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3117/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3117(t1168: f64, t12423: f64, t12429: f64, t12470: f64, t12511: f64, t17032: f64, t1744: f64, t1745: f64, t20520: f64, t20542: f64, t20612: f64, t20618: f64, t20622: f64, t20626: f64, t24331: f64, t24366: f64, t24417: f64, t24420: f64, t3452: f64, t3477: f64, t45085: f64, t5142: f64, t5143: f64, t58005: f64, t58304: f64, t6487: f64, t6502: f64, t6506: f64, t69411: f64, t69565: f64) -> f64 {
    let t82045 = -0.57895126195293126241e3_f64 * t58304 * t20612 + 0.1929837539843104208e3_f64 * t17032 * t20622 + 3.0_f64 * t69565 * t1745 + 3.0_f64 * t20542 * t5143 + 18.0_f64 * t3477 * t6487 * t5142 - 6.0_f64 * t12511 * t24417 - 6.0_f64 * t3452 * t5143 * t6502 - 6.0_f64 * t3452 * t1745 * t20520 + 0.96491876992155210402e2_f64 * t12423 * t24420 + 0.96491876992155210402e2_f64 * t3477 * t69411 * t1744 + 0.96491876992155210402e2_f64 * t3477 * t20618 * t5142 + 0.62071215503128080361e4_f64 * t58005 * t20626 + 0.11579025239058625248e4_f64 * t12470 * t24331 * t1168 - 0.57895126195293126243e3_f64 * t12429 * t6506 * t5142 - 0.24828486201251232145e5_f64 * t45085 * t24366 * t1168;
    t82045
}
