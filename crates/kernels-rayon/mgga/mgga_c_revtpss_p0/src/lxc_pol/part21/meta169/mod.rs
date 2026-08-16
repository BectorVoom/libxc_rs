//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta169 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1069;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1070;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta169(t124: f64, t3829: f64, t800: f64, t1376: f64, t2689: f64, t1353: f64, t1413: f64, t547: f64, t807: f64, t2700: f64, t535: f64, t1369: f64, t794: f64, t1372: f64, t3889: f64, t2453: f64, t546: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3945, t3946, t3950, t3951, t3952, t3953, t3956, t3957) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1069(t124, t3829, t800, t1376, t2689, t1353, t1413, t547, t807, t2700, t535, t1369, t794);
        let (t3958, t3960, t3961, t3964) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1070(t1372, t3957, t124, t3889, t800, t2453, t546);
    (t3945, t3946, t3950, t3951, t3952, t3953, t3956, t3957, t3958, t3960, t3961, t3964)
}
