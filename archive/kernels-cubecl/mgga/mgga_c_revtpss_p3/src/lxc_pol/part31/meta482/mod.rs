//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta482 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1762;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1763;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1764;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1765;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1766;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1767;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta482<F: Float>(t122: F, t1949: F, t72: F, t2466: F, t25375: F, t1955: F, t25308: F, t251: F, t7063: F, t25374: F, t2769: F, t7056: F, t822: F, t1950: F, t867: F, t786: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25377, t25378, t25379, t25383) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1762::<F>(t122, t1949, t72, t2466, t25375, t1955, t25308);
        let t25386 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1763::<F>(t251, t7063);
        let t25387 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1764::<F>(t25374, t25386);
        let (t25388, t25390, t25391) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1765::<F>(t25378, t25387, t2769, t7056, t1955);
        let t25392 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1766::<F>(t1949, t822);
        let (t25398, t25399) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1767::<F>(t1950, t867, t786);
    (t25377, t25378, t25379, t25383, t25386, t25387, t25388, t25390, t25391, t25392, t25398, t25399)
}
