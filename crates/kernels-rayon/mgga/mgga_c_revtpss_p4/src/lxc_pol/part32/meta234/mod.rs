//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta234 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk996;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk997;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta234(t1045: f64, t373: f64, t6299: f64, t1042: f64, t1668: f64, t3155: f64, t3162: f64, t225: f64, t6235: f64, t366: f64, t1066: f64, t6100: f64, t247: f64, t3182: f64, t6092: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6301, t6302, t6305) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk996(t1045, t373, t6299, t1042, t1668);
        let (t6307, t6308, t6311, t6312, t6317, t6318, t6323, t6326) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk997(t373, t6305, t3155, t1042, t3162, t225, t6235, t366, t1066, t6100, t247, t3182, t6092);
    (t6301, t6302, t6305, t6307, t6308, t6311, t6312, t6317, t6318, t6323, t6326)
}
