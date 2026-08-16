//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta473 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1785;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1786;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1787;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1788;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1789;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1790;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta473<F: Float>(t25375: F, t25378: F, t1955: F, t25308: F, t251: F, t7063: F, t25374: F, t2769: F, t7056: F, t1949: F, t822: F, t231: F, t836: F, t886: F, t1950: F, t867: F, t786: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25379, t25383) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1785::<F>(t25375, t25378, t1955, t25308);
        let t25386 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1786::<F>(t251, t7063);
        let t25387 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1787::<F>(t25374, t25386);
        let (t25388, t25390, t25391) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1788::<F>(t25378, t25387, t2769, t7056, t1955);
        let t25392 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1789::<F>(t1949, t822);
        let (t25394, t25395, t25398, t25399) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1790::<F>(t231, t836, t886, t25392, t1950, t867, t786);
    (t25379, t25383, t25386, t25387, t25388, t25390, t25391, t25392, t25394, t25395, t25398, t25399)
}
