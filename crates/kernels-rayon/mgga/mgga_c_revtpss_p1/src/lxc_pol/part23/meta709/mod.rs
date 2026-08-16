//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta709 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2464;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2465;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta709(t1904: f64, t2439: f64, t9640: f64, t5718: f64, t9292: f64, t14274: f64, t2435: f64, t10175: f64, t14090: f64, t14085: f64, t14104: f64, t47520: f64, t10069: f64, t13731: f64, t137: f64, t14103: f64, t47480: f64, t9675: f64, t14099: f64, t2453: f64, t9676: f64, t14109: f64, t9680: f64, t9685: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47800, t47802, t47806, t47814, t47835, t47837) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2464(t1904, t2439, t9640, t5718, t9292, t14274, t2435, t10175, t14090, t14085, t14104, t47520);
        let (t47838, t47839, t47845, t47856, t47858, t47860) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2465(t47837, t10069, t13731, t137, t14103, t47480, t9675, t14099, t2453, t9676, t14109, t9680, t9685);
    (t47800, t47802, t47806, t47814, t47835, t47838, t47839, t47845, t47856, t47858, t47860)
}
