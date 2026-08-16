//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta275 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1234;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1235;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1236;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1237;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta275(t2035: f64, t7898: f64, t1450: f64, t1868: f64, t7237: f64, t2014: f64, t1873: f64, t7252: f64, t1885: f64, t7264: f64, t1889: f64, t7271: f64, t7251: f64, t7258: f64, t7261: f64, t7268: f64, t225: f64, t1892: f64, t1955: f64, t1903: f64, t2022: f64, t7296: f64, t1882: f64, t543: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7899, t7900, t7901, t7903, t7904, t7906, t7908) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1234(t2035, t7898, t1450, t1868, t7237, t2014, t1873, t7252, t1885, t7264, t1889, t7271);
        let t7910 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1235(t7251, t7258, t7261, t7268, t7904, t7906, t7908);
        let (t7911, t7917, t7920) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1236(t225, t7910, t1892, t1955, t1903, t2022);
        let (t7921, t7925) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1237(t7296, t7920, t1882, t2022, t543);
    (t7899, t7900, t7901, t7903, t7910, t7911, t7917, t7920, t7921, t7925)
}
