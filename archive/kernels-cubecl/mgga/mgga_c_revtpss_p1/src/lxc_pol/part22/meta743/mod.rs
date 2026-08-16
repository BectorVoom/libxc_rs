//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta743 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2810;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2811;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta743<F: Float>(t11043: F, t2453: F, t10506: F, t2458: F, t2761: F, t11029: F, t9303: F, t39501: F, t781: F, t10510: F, t11044: F, t252: F, t257: F, t268: F, t39644: F, t8779: F, t10995: F, t123: F, t2434: F, t2771: F, t10504: F, t138: F, t2438: F, t2828: F, t11015: F, t2461: F, t2769: F, t786: F, t861: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t41020, t41021, t41029, t41034, t41037, t41038, t41049) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2810::<F>(t11043, t2453, t10506, t2458, t2761, t11029, t9303, t39501, t781, t10510, t11044, t252, t257, t268, t39644, t8779);
        let (t41052, t41056, t41060, t41066) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2811::<F>(t10995, t123, t2434, t2771, t10504, t138, t2438, t2828, t11015, t2461, t2769, t786, t861);
    (t41020, t41021, t41029, t41034, t41037, t41038, t41049, t41052, t41056, t41060, t41066)
}
