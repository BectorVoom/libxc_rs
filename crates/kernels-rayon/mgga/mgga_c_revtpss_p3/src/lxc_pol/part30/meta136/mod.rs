//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta136 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk743;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk744;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk745;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk746;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta136(t2986: f64, t315: f64, t972: f64, t973: f64, t2846: f64, t2904: f64, t2848: f64, t2855: f64, t2860: f64, t2864: f64, t2882: f64, t2890: f64, t2898: f64, t2900: f64, t2906: f64, t2910: f64, t2913: f64, t2916: f64, t963: f64, t323: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2987, t2988) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk743(t2986, t315, t972);
        let (t2989, t2994, t3001, t3006) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk744(t2988, t973, t2846, t2904, t2848, t2855, t2860, t2864, t2882, t2890, t2898, t2900, t2906, t2910, t2913, t2916);
        let (t3007, t3010, t3011) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk745(t3006, t973, t963);
        let (t3012, t3013, t3014) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk746(t3011, t315, t323);
    (t2987, t2988, t2989, t2994, t3001, t3006, t3007, t3010, t3011, t3012, t3013, t3014)
}
