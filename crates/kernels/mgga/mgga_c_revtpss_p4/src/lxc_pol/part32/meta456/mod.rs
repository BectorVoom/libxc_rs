//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta456 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1656;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1657;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1658;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1659;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1660;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1661;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta456<F: Float>(t25304: F, t7057: F, t1032: F, t860: F, t867: F, t786: F, t11007: F, t233: F, t7063: F, t251: F, t2769: F, t1955: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25305, t25308, t25309, t25310, t25317) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1656::<F>(t25304, t7057, t1032, t860, t867, t786, t11007, t233);
        let (t25365, t25372) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1657::<F>(t25309, t7063, t251, t786);
        let (t25373, t25374) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1658::<F>(t1032, t2769, t233);
        let t25375 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1659::<F>(t25372, t25374);
        let t25383 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1660::<F>(t1955, t25308);
        let t25386 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1661::<F>(t251, t7063);
    (t25305, t25308, t25309, t25310, t25317, t25365, t25372, t25373, t25374, t25375, t25383, t25386)
}
