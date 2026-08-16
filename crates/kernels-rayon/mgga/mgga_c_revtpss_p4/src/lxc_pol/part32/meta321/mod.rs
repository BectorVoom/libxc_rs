//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta321 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1238;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1239;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta321(t2434: f64, t371: f64, t373: f64, t367: f64, t1065: f64, t675: f64, t247: f64, t906: f64, t1063: f64, t1062: f64, t3223: f64, t1052: f64, t3147: f64, t1036: f64, t3141: f64, t3144: f64, t1035: f64, t11239: f64, t342: f64, t3145: f64, t334: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11972, t11986, t11989, t11994, t11997) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1238(t2434, t371, t373, t367, t1065, t675, t247, t906, t1063, t1062, t3223, t1052, t3147);
        let (t11999, t12013, t12046, t12047, t12050) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1239(t1036, t11997, t3141, t3144, t1035, t11239, t342, t3145, t334);
    (t11972, t11986, t11989, t11994, t11999, t12013, t12046, t12047, t12050)
}
