//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3020/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3020(t12050: f64, t357: f64, t11631: f64, t6299: f64, t1043: f64, t11940: f64, t12047: f64, t12052: f64, t16502: f64, t16552: f64, t16559: f64, t16560: f64, t16566: f64, t19450: f64, t19456: f64, t19502: f64, t20123: f64, t20139: f64, t20146: f64, t43341: f64, t43438: f64, t4866: f64, t4954: f64, t5004: f64, t5012: f64, t55499: f64, t55646: f64, t55887: f64, t6235: f64, t6365: f64, t78496: f64, t80277: f64, t80312: f64, t80341: f64, t999: f64) -> f64 {
    let t80350 = t12050 * t357;
    let t80358 = t11631 * t6299;
    let t80391 = 0.19756347548806534796e1_f64 * t16566 * t19450 * t80350 * t4866 + 0.79025390195226139182e1_f64 * t43438 * t80312 * t19502 + 0.11853808529283920877e2_f64 * t16552 * t55499 * t80358 * t1043 - 0.11853808529283920877e2_f64 * t16559 * t55499 * t80277 * t1043 - 0.11853808529283920877e2_f64 * t11940 * t5004 * t19456 + 0.39512695097613069591e1_f64 * t4954 * t20123 - 0.39512695097613069591e1_f64 * t55646 * t6365 - 0.39512695097613069591e1_f64 * t16502 * t20146 - 0.11853808529283920877e2_f64 * t16559 * t19450 * t16560 * t4866 + 0.39512695097613069591e1_f64 * t55887 * t20139 - 0.65854491829355115987e0_f64 * t43341 * t78496 * t80350 * t999 + 0.65854491829355115987e0_f64 * t12047 * t80341 * t12052 + 0.19756347548806534796e1_f64 * t6235 * t5012;
    t80391
}
