//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 904/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk904<F: Float>(t31772: F, t4364: F, t886: F, t31767: F, t1032: F, t8471: F, t867: F, t786: F, t233: F, t72: F, t686: F, t7063: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31774 = t4364 * t31772 * t886;
    let t31775 = t31767 * t31774;
    let t31777 = t8471 * t1032;
    let t31778 = t31777 * t867;
    let t31779 = t786 * t31778;
    let t31780 = t233 * t72;
    let t31781 = t31780 * t686;
    let t31783 = F::new(0.14456046980341999104e-1) * t31779 * t31781;
    let t31784 = t7063 * t31778;
    (t31774, t31775, t31777, t31778, t31779, t31780, t31781, t31783, t31784)
}
