//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta277 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1240;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1241;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1242;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1243;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta277<F: Float>(t2035: F, t7898: F, t1450: F, t1868: F, t7237: F, t2014: F, t1873: F, t7252: F, t1885: F, t7264: F, t1889: F, t7271: F, t7251: F, t7258: F, t7261: F, t7268: F, t225: F, t1892: F, t1955: F, t1903: F, t2022: F, t7296: F, t1882: F, t543: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t7899, t7900, t7901, t7903, t7904, t7906, t7908) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1240::<F>(t2035, t7898, t1450, t1868, t7237, t2014, t1873, t7252, t1885, t7264, t1889, t7271);
        let t7910 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1241::<F>(t7251, t7258, t7261, t7268, t7904, t7906, t7908);
        let (t7911, t7917, t7920) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1242::<F>(t225, t7910, t1892, t1955, t1903, t2022);
        let (t7921, t7925) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1243::<F>(t7296, t7920, t1882, t2022, t543);
    (t7899, t7900, t7901, t7903, t7910, t7911, t7917, t7920, t7921, t7925)
}
