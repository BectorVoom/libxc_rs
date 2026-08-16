//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta96 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk613;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk614;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk615;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk616;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta96(t33: f64, t2002: f64, t2159: f64, t57: f64, t2132: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t2010: f64, t2127: f64, t118: f64, t1939: f64, t2036: f64, t508: f64, t569: f64, t3: f64, param_d: f64, t2044: f64, t573: f64, t10: f64, t17: f64, t576: f64, t580: f64, t15: f64, t22: f64, t11: f64, t14: f64, t584: f64, t588: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2163 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk613(t33, t2002, t2159, t57, t2132, dens_threshold, rho1, zeta_threshold);
        let t2165 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk614(t2010, t2127);
        let (t2167, t2168, t2170) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk615(t118, t1939, t2036, t2127, t2163, t2165, t508, t569, t3, param_d);
        let (t2172, t2219, t2221, t2223, t2224, t2226, t2228) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk616(t2044, t2170, t573, t10, t17, t576, t580, t15, t22, t11, t14, t584, t588);
    (t2163, t2165, t2167, t2168, t2170, t2172, t2219, t2221, t2223, t2224, t2226, t2228)
}
