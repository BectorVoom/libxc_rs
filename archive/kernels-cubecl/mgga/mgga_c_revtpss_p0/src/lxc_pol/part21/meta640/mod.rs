//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta640 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2415;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2416;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2417;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta640<F: Float>(t11331: F, t698: F, t2439: F, t2912: F, t11328: F, t2915: F, t2909: F, t11345: F, t11342: F, t11821: F, t240: F, t2851: F, t25273: F, t268: F, t271: F, t11161: F, t689: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t41275, t41281, t41283, t41285, t41287, t41289, t41292, t41294, t41295) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2415::<F>(t11331, t698, t2439, t2912, t11328, t2915, t2909, t11345, t11342, t11821, t240, t2851);
        let (t41296, t41306) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2416::<F>(t41295, t25273, t268, t271);
        let (t41307, t41308) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2417::<F>(t41306, t11161, t689);
    (t41275, t41281, t41283, t41285, t41287, t41289, t41292, t41294, t41296, t41306, t41307, t41308)
}
