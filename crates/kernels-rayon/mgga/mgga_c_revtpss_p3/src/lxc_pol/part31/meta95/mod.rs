//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta95 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk611;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk612;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk613;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta95(t2014: f64, t2035: f64, t118: f64, t1932: f64, t1939: f64, t2007: f64, t2011: f64, t508: f64, t569: f64, t3: f64, param_d: f64, t117: f64, t1936: f64, t572: f64, t573: f64, t10: f64, t17: f64, t576: f64, t580: f64, t15: f64, t22: f64, t11: f64, t14: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2037, t2038, t2040) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk611(t2014, t2035, t118, t1932, t1939, t2007, t2011, t508, t569, t3, param_d);
        let t2042 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk612(t117, t1936);
        let (t2045, t2219, t2221, t2223, t2224) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk613(t2042, t572, t2040, t573, t10, t17, t576, t580, t15, t22, t11, t14);
    (t2037, t2038, t2040, t2042, t2045, t2219, t2221, t2223, t2224)
}
