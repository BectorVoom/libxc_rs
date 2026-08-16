//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3028/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3028(t342: f64, t378: f64, t43346: f64, t42872: f64, t43350: f64, t12046: f64, t1647: f64, t1082: f64, t11247: f64, t12053: f64, t12078: f64, t12079: f64, t12116: f64, t16427: f64, t16443: f64, t16446: f64, t16520: f64, t16555: f64, t16562: f64, t3223: f64, t42359: f64, t43154: f64, t4961: f64, t53670: f64, t54983: f64, t55569: f64, t55570: f64, t55575: f64, t55579: f64, t55583: f64, t55586: f64) -> f64 {
    let t55593 = t342 * t43346 * t378;
    let t55594 = t43350 * t42872;
    let t55599 = t1647 * t12046;
    let t55607 = 0.79025390195226139182e1_f64 * t16520 * t16443 + 0.39512695097613069591e1_f64 * t12116 * t16427 - 0.23707617058567841754e2_f64 * t55569 * t53670 * t55570 * t11247 - 0.11853808529283920877e2_f64 * t55575 * t16562 + 0.11853808529283920877e2_f64 * t55579 * t16555 - 0.11853808529283920877e2_f64 * t55583 * t16562 - 0.39512695097613069591e1_f64 * t12078 * t55586 * t12079 + 0.39512695097613069591e1_f64 * t42359 * t4961 + 0.15805078039045227836e2_f64 * t55593 * t53670 * t55594 * t11247 + 0.65854491829355115987e0_f64 * t55599 * t12053 - 0.19756347548806534796e1_f64 * t3223 * t16446 + 0.15805078039045227836e2_f64 * t43154 * t1082 * t54983;
    t55607
}
