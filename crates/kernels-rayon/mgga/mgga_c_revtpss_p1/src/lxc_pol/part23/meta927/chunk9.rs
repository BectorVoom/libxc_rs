//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3018/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3018(t3154: f64, t6299: f64, t12050: f64, t12122: f64, t16432: f64, t16566: f64, t16584: f64, t19414: f64, t19491: f64, t19534: f64, t19548: f64, t19584: f64, t19594: f64, t19597: f64, t24116: f64, t24162: f64, t3204: f64, t3278: f64, t342: f64, t380: f64, t43520: f64, t43524: f64, t4954: f64, t5004: f64, t55732: f64, t55958: f64, t56049: f64, t6375: f64, t65144: f64, t79703: f64, t80132: f64, t80248: f64, t989: f64, t999: f64) -> (f64, f64) {
    let t80277 = t3154 * t6299;
    let t80310 = 0.39512695097613069591e1_f64 * t4954 * t19584 - 0.39512695097613069591e1_f64 * t12122 * t16432 * t80277 * t999 + 0.19756347548806534796e1_f64 * t16566 * t65144 * t12050 * t79703 - 0.79025390195226139182e1_f64 * t56049 * t19594 + 0.39512695097613069591e1_f64 * t55732 * t19597 - 0.19756347548806534796e1_f64 * t16584 * t19534 + 0.65854491829355115987e0_f64 * t989 * t24162 + 0.19756347548806534796e1_f64 * t3278 * t24116 + 0.65854491829355115987e0_f64 * t342 * t380 * t80132 + 0.39512695097613069592e1_f64 * t3204 * t5004 * t19414 + 0.39512695097613069591e1_f64 * t55958 * t6375 - 0.11853808529283920877e2_f64 * t43520 * t80248 * t19548 + 0.11853808529283920877e2_f64 * t43524 * t80248 * t19491;
    (t80277, t80310)
}
