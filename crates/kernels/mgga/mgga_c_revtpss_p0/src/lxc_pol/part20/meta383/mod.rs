//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta383 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1397;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1398;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1399;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1400;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1401;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta383<F: Float>(t11025: F, t2435: F, t10981: F, t588: F, t780: F, t10991: F, t39497: F, t787: F, t788: F, t2448: F, t9292: F, t11036: F, t10910: F, t213: F, t10994: F, t2453: F, t138: F, t2438: F, t2771: F, t2761: F, t786: F, t867: F, t2467: F, t11043: F, t10506: F, t10495: F, t11008: F, t2765: F, t2828: F, t40978: F, t40982: F, t40986: F, t40988: F, t865: F, t887: F, t11032: F, t789: F, t2458: F, t2444: F, t2772: F, t689: F, t11029: F, t9303: F, t39501: F, t781: F, t10510: F, t11044: F, t675: F, t886: F, t10995: F, t268: F, t252: F, t257: F, t39644: F, t8779: F, t123: F, t2434: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t40994, t40998, t40999, t41003, t41004, t41006) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1397::<F>(t11025, t2435, t10981, t588, t780, t10991, t39497, t787, t788, t2448, t9292, t11036);
        let (t41008, t41014, t41018, t41020) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1398::<F>(t10910, t213, t10994, t2453, t138, t2438, t2771, t2761, t786, t867, t2467, t11043);
        let t41023 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1399::<F>(t10506, t41020, t10495, t11008, t2765, t2771, t2828, t40978, t40982, t40986, t40988, t40994, t40998, t40999, t41003, t41004, t41006, t41008, t41014, t41018, t865, t887);
        let (t41026, t41029, t41032, t41034, t41037) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1400::<F>(t11032, t786, t789, t2453, t2458, t2761, t2444, t2772, t689, t11029, t9303, t39501, t781);
        let (t41038, t41043, t41049, t41052) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1401::<F>(t10510, t11044, t675, t886, t10995, t268, t2828, t252, t257, t39644, t8779, t123, t2434, t2771);
    (t41023, t41026, t41029, t41032, t41034, t41037, t41038, t41043, t41049, t41052)
}
