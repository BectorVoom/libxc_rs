//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta383 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1397;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1398;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1399;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1400;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1401;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta383(t11025: f64, t2435: f64, t10981: f64, t588: f64, t780: f64, t10991: f64, t39497: f64, t787: f64, t788: f64, t2448: f64, t9292: f64, t11036: f64, t10910: f64, t213: f64, t10994: f64, t2453: f64, t138: f64, t2438: f64, t2771: f64, t2761: f64, t786: f64, t867: f64, t2467: f64, t11043: f64, t10506: f64, t10495: f64, t11008: f64, t2765: f64, t2828: f64, t40978: f64, t40982: f64, t40986: f64, t40988: f64, t865: f64, t887: f64, t11032: f64, t789: f64, t2458: f64, t2444: f64, t2772: f64, t689: f64, t11029: f64, t9303: f64, t39501: f64, t781: f64, t10510: f64, t11044: f64, t675: f64, t886: f64, t10995: f64, t268: f64, t252: f64, t257: f64, t39644: f64, t8779: f64, t123: f64, t2434: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40994, t40998, t40999, t41003, t41004, t41006) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1397(t11025, t2435, t10981, t588, t780, t10991, t39497, t787, t788, t2448, t9292, t11036);
        let (t41008, t41014, t41018, t41020) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1398(t10910, t213, t10994, t2453, t138, t2438, t2771, t2761, t786, t867, t2467, t11043);
        let t41023 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1399(t10506, t41020, t10495, t11008, t2765, t2771, t2828, t40978, t40982, t40986, t40988, t40994, t40998, t40999, t41003, t41004, t41006, t41008, t41014, t41018, t865, t887);
        let (t41026, t41029, t41032, t41034, t41037) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1400(t11032, t786, t789, t2453, t2458, t2761, t2444, t2772, t689, t11029, t9303, t39501, t781);
        let (t41038, t41043, t41049, t41052) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1401(t10510, t11044, t675, t886, t10995, t268, t2828, t252, t257, t39644, t8779, t123, t2434, t2771);
    (t41023, t41026, t41029, t41032, t41034, t41037, t41038, t41043, t41049, t41052)
}
