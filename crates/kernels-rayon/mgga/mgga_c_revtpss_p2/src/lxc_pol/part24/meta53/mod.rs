//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta53 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk356;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk357;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk358;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk359;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta53(t1320: f64, t521: f64, t520: f64, t749: f64, t512: f64, t72: f64, t757: f64, t177: f64, t762: f64, t531: f64, t566: f64, t513: f64, t516: f64, t212: f64, t555: f64, t225: f64, t561: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1322, t1333) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk356(t1320, t521, t520, t749);
        let (t1334, t1337, t1339, t1340) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk357(t1333, t512, t520, t72, t757, t177);
        let (t1342, t1343, t1344, t1348, t1357) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk358(t1340, t762, t531, t566, t513, t516, t212, t555);
        let t1358 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk359(t225, t561);
    (t1322, t1333, t1334, t1337, t1339, t1340, t1342, t1343, t1344, t1348, t1357, t1358)
}
