//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta220 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk969;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk970;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk971;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta220(t1086: f64, t3057: f64, t3090: f64, t11200: f64, t225: f64, t366: f64, t2434: f64, t371: f64, t373: f64, t367: f64, t1065: f64, t675: f64, t1035: f64, t11239: f64, t342: f64, t3145: f64, t334: f64, t11249: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11926, t11927, t11940) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk969(t1086, t3057, t3090, t11200, t225);
        let (t11941, t11970, t11972, t11986, t12046, t12047, t12050) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk970(t11940, t366, t2434, t371, t373, t367, t1065, t675, t1035, t11239, t342, t3145, t334);
        let t12051 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk971(t11249, t12050);
    (t11926, t11927, t11940, t11941, t11970, t11972, t11986, t12046, t12047, t12050, t12051)
}
