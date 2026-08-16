//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1797/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1797(t6628: f64, t6695: f64, t487: f64, t90080: f64, t12050: f64, t1287: f64, t1774: f64, t17846: f64, t17847: f64, t17854: f64, t17949: f64, t20956: f64, t21439: f64, t3755: f64, t3767: f64, t3769: f64, t3782: f64, t3783: f64, t45654: f64, t45659: f64, t471: f64, t6622: f64, t6735: f64, t70890: f64, t82293: f64, t90054: f64, t91199: f64) -> f64 {
    let t91492 = t6695 * t6628;
    let t91501 = t487 * t90080;
    let t91513 = 0.23707617058567841754e2_f64 * t17846 * t20956 * t17847 * t6622 - 0.15805078039045227836e2_f64 * t45654 * t82293 * t17847 * t1774 + 0.15805078039045227836e2_f64 * t45659 * t82293 * t17854 * t1774 - 0.26341796731742046395e1_f64 * t3755 * t90054 * t1287 - 0.39512695097613069592e1_f64 * t3755 * t91199 * t1287 - 0.39512695097613069592e1_f64 * t3782 * t91492 * t3783 + 0.39512695097613069592e1_f64 * t21439 * t6735 + 0.79025390195226139183e1_f64 * t3767 * t91492 * t3769 - 0.19756347548806534796e1_f64 * t3782 * t91501 * t3783 + 0.39512695097613069591e1_f64 * t3767 * t91501 * t3769 + 0.39512695097613069592e1_f64 * t17949 * t70890 * t12050 * t6628 * t471;
    t91513
}
