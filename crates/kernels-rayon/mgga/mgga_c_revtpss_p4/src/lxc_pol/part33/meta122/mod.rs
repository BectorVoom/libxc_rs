//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta122 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk693;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk694;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk695;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta122(t2969: f64, t2846: f64, t960: f64, t964: f64, t320: f64, t963: f64, t315: f64, t2904: f64, t323: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2970, t2974, t2982, t2986) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk693(t2969, t2846, t960, t964, t320, t963);
        let (t2987, t2994, t3001, t3010, t3011) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk694(t2986, t315, t2846, t2904, t963);
        let (t3012, t3013, t3014) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk695(t3011, t315, t323);
    (t2970, t2974, t2982, t2986, t2987, t2994, t3001, t3010, t3011, t3012, t3013, t3014)
}
