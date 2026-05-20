//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3028/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3028<F: Float>(t342: F, t378: F, t43346: F, t42872: F, t43350: F, t12046: F, t1647: F, t1082: F, t11247: F, t12053: F, t12078: F, t12079: F, t12116: F, t16427: F, t16443: F, t16446: F, t16520: F, t16555: F, t16562: F, t3223: F, t42359: F, t43154: F, t4961: F, t53670: F, t54983: F, t55569: F, t55570: F, t55575: F, t55579: F, t55583: F, t55586: F) -> F {
    let t55593 = t342 * t43346 * t378;
    let t55594 = t43350 * t42872;
    let t55599 = t1647 * t12046;
    let t55607 = F::cast_from(0.79025390195226139182e1_f64) * t16520 * t16443 + F::cast_from(0.39512695097613069591e1_f64) * t12116 * t16427 - F::cast_from(0.23707617058567841754e2_f64) * t55569 * t53670 * t55570 * t11247 - F::cast_from(0.11853808529283920877e2_f64) * t55575 * t16562 + F::cast_from(0.11853808529283920877e2_f64) * t55579 * t16555 - F::cast_from(0.11853808529283920877e2_f64) * t55583 * t16562 - F::cast_from(0.39512695097613069591e1_f64) * t12078 * t55586 * t12079 + F::cast_from(0.39512695097613069591e1_f64) * t42359 * t4961 + F::cast_from(0.15805078039045227836e2_f64) * t55593 * t53670 * t55594 * t11247 + F::cast_from(0.65854491829355115987e0_f64) * t55599 * t12053 - F::cast_from(0.19756347548806534796e1_f64) * t3223 * t16446 + F::cast_from(0.15805078039045227836e2_f64) * t43154 * t1082 * t54983;
    t55607
}
