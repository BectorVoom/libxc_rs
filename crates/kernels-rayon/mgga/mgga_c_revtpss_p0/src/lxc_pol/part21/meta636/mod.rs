//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta636 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2408;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2409;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta636(t11029: f64, t9303: f64, t39501: f64, t781: f64, t10510: f64, t11044: f64, t675: f64, t886: f64, t10995: f64, t268: f64, t2828: f64, t252: f64, t257: f64, t39644: f64, t8779: f64, t123: f64, t2434: f64, t2771: f64, t10504: f64, t138: f64, t2438: f64, t11050: f64, t11015: f64, t2461: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41034, t41037, t41038, t41043, t41049) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2408(t11029, t9303, t39501, t781, t10510, t11044, t675, t886, t10995, t268, t2828, t252, t257, t39644, t8779);
        let (t41052, t41056, t41058, t41060) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2409(t10995, t123, t2434, t2771, t10504, t138, t2438, t2828, t11044, t11050, t11015, t2461);
    (t41034, t41037, t41038, t41043, t41049, t41052, t41056, t41058, t41060)
}
