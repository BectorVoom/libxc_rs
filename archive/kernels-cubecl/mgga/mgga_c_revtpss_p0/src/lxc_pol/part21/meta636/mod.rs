//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta636 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2408;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2409;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta636<F: Float>(t11029: F, t9303: F, t39501: F, t781: F, t10510: F, t11044: F, t675: F, t886: F, t10995: F, t268: F, t2828: F, t252: F, t257: F, t39644: F, t8779: F, t123: F, t2434: F, t2771: F, t10504: F, t138: F, t2438: F, t11050: F, t11015: F, t2461: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t41034, t41037, t41038, t41043, t41049) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2408::<F>(t11029, t9303, t39501, t781, t10510, t11044, t675, t886, t10995, t268, t2828, t252, t257, t39644, t8779);
        let (t41052, t41056, t41058, t41060) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2409::<F>(t10995, t123, t2434, t2771, t10504, t138, t2438, t2828, t11044, t11050, t11015, t2461);
    (t41034, t41037, t41038, t41043, t41049, t41052, t41056, t41058, t41060)
}
