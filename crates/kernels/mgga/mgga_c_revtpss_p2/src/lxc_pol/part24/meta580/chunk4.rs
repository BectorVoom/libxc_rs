//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1797/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1797<F: Float>(t6628: F, t6695: F, t487: F, t90080: F, t12050: F, t1287: F, t1774: F, t17846: F, t17847: F, t17854: F, t17949: F, t20956: F, t21439: F, t3755: F, t3767: F, t3769: F, t3782: F, t3783: F, t45654: F, t45659: F, t471: F, t6622: F, t6735: F, t70890: F, t82293: F, t90054: F, t91199: F) -> F {
    let t91492 = t6695 * t6628;
    let t91501 = t487 * t90080;
    let t91513 = F::cast_from(0.23707617058567841754e2_f64) * t17846 * t20956 * t17847 * t6622 - F::cast_from(0.15805078039045227836e2_f64) * t45654 * t82293 * t17847 * t1774 + F::cast_from(0.15805078039045227836e2_f64) * t45659 * t82293 * t17854 * t1774 - F::cast_from(0.26341796731742046395e1_f64) * t3755 * t90054 * t1287 - F::cast_from(0.39512695097613069592e1_f64) * t3755 * t91199 * t1287 - F::cast_from(0.39512695097613069592e1_f64) * t3782 * t91492 * t3783 + F::cast_from(0.39512695097613069592e1_f64) * t21439 * t6735 + F::cast_from(0.79025390195226139183e1_f64) * t3767 * t91492 * t3769 - F::cast_from(0.19756347548806534796e1_f64) * t3782 * t91501 * t3783 + F::cast_from(0.39512695097613069591e1_f64) * t3767 * t91501 * t3769 + F::cast_from(0.39512695097613069592e1_f64) * t17949 * t70890 * t12050 * t6628 * t471;
    t91513
}
