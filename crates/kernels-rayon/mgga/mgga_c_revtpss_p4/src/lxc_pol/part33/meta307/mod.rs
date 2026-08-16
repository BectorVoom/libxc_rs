//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta307 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1303;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1304;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta307(t202: f64, t57: f64, t2435: f64, t2445: f64, t2441: f64, t9303: f64, t10115: f64, t258: f64, t2453: f64, t2464: f64, t2438: f64, t886: f64, t138: f64, t2434: f64, t123: f64, t2465: f64, t215: f64, t231: f64, t268: f64, t836: f64, t2798: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10457, t10498, t10501, t10503, t10504, t10505) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1303(t202, t57, t2435, t2445, t2441, t9303, t10115, t258, t2453, t2464, t2438, t886);
        let (t10506, t10507, t10509, t10510, t10511, t10519) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1304(t10505, t138, t10504, t2434, t886, t123, t2465, t215, t231, t268, t836, t2798);
    (t10457, t10498, t10501, t10503, t10504, t10505, t10506, t10507, t10509, t10510, t10511, t10519)
}
