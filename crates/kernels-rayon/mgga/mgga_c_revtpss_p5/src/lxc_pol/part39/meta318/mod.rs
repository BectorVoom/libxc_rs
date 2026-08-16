//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta318 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1090;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1091;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta318(t3241: f64, t3244: f64, t1058: f64, t3197: f64, t11132: f64, t3163: f64, t3172: f64, t3161: f64, t126: f64, t373: f64, t828: f64, t3119: f64, t3115: f64, t1086: f64, t3057: f64, t3090: f64, t11671: f64, t3114: f64, t127: f64, t3206: f64, t371: f64, t3205: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11886, t11888, t11890, t11917, t11921, t11922) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1090(t3241, t3244, t1058, t3197, t11132, t3163, t3172, t3161, t126, t373, t828);
        let (t11924, t11927, t11933, t11938) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1091(t11922, t3119, t3115, t1086, t3057, t3090, t11671, t3114, t127, t3206, t371, t3205);
    (t11886, t11888, t11890, t11917, t11921, t11922, t11924, t11927, t11933, t11938)
}
