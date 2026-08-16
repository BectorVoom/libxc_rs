//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta279 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1246;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1247;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1248;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta279(t3: f64, t7939: f64, t1916: f64, t2042: f64, t1518: f64, t7330: f64, t572: f64, t117: f64, t7741: f64, t1918: f64, t2040: f64, t573: f64, param_d: f64, t3140: f64, t3268: f64, t1078: f64, t1035: f64, t2033: f64, t4147: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7940, t7944, t7950, t7953, t7956) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1246(t3, t7939, t1916, t2042, t1518, t7330, t572, t117, t7741, t1918, t2040, t573, param_d);
        let (t8515, t8521) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1247(t3140, t3268, t1078, t1035);
        let t8717 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1248(t2033, t4147);
    (t7940, t7944, t7950, t7953, t7956, t8515, t8521, t8717)
}
