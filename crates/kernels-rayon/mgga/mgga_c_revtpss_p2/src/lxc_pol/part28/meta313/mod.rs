//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta313 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1316;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta313(t2435: f64, t2445: f64, t2441: f64, t9303: f64, t10115: f64, t258: f64, t2453: f64, t2464: f64, t2438: f64, t886: f64, t138: f64, t2434: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10498, t10501, t10503, t10504, t10505, t10506, t10507, t10509) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1316(t2435, t2445, t2441, t9303, t10115, t258, t2453, t2464, t2438, t886, t138, t2434);
    (t10498, t10501, t10503, t10504, t10505, t10506, t10507, t10509)
}
