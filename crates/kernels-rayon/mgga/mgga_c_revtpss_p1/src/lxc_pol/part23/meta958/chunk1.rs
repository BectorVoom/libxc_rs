//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3214/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3214(t24864: f64, t473: f64, t11249: f64, t24834: f64, t3153: f64, t1214: f64, t1234: f64, t17183: f64, t17846: f64, t17847: f64, t20956: f64, t21416: f64, t21439: f64, t21465: f64, t21468: f64, t21472: f64, t21500: f64, t21541: f64, t21562: f64, t21579: f64, t21582: f64, t21586: f64, t21596: f64, t3670: f64, t45654: f64, t45659: f64, t45863: f64, t5230: f64, t5284: f64, t5436: f64, t5470: f64) -> (f64, f64, f64) {
    let t84429 = t473 * t24864;
    let t84450 = t24834 * t11249;
    let t84457 = t24834 * t3153;
    let t84461 = -0.19756347548806534796e1_f64 * t17183 * t21416 - 0.65854491829355115987e0_f64 * t1234 * t84429 * t1214 + 0.39512695097613069592e1_f64 * t3670 * t21541 * t5230 + 0.79025390195226139182e1_f64 * t21500 * t21596 + 0.19756347548806534796e1_f64 * t5436 * t21562 + 0.19756347548806534796e1_f64 * t21439 * t5470 + 0.39512695097613069592e1_f64 * t21500 * t21465 - 0.19756347548806534796e1_f64 * t21579 * t21468 + 0.11853808529283920877e2_f64 * t17846 * t20956 * t17847 * t5284 - 0.11853808529283920877e2_f64 * t45654 * t84450 * t21582 + 0.11853808529283920877e2_f64 * t45659 * t84450 * t21586 - 0.39512695097613069592e1_f64 * t45863 * t84457 * t21472;
    (t84450, t84457, t84461)
}
