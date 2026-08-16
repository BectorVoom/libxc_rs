//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta409 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1489;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1490;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta409(t10208: f64, t104: f64, t69: f64, t2339: f64, t2681: f64, t64: f64, t10207: f64, t111: f64, t116: f64, t13424: f64, t1501: f64, t2371: f64, t4245: f64, t670: f64, t1518: f64, t2319: f64, t4292: f64, t648: f64, t13514: f64, t94: f64, t1513: f64, t2340: f64, t4287: f64, t665: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t36308, t36315, t46089, t46157, t49686, t75485) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1489(t10208, t104, t69, t2339, t2681, t64, t10207, t111, t116, t13424, t1501, t2371);
        let (t75667, t98484, t98487, t98535, t101457, t101460) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1490(t4245, t670, t1518, t2319, t4292, t648, t13514, t94, t1513, t2340, t4287, t665);
    (t36308, t36315, t46089, t46157, t49686, t75485, t75667, t98484, t98487, t98535, t101457, t101460)
}
