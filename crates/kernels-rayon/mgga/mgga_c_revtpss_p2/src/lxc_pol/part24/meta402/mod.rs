//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta402 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1337;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1338;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta402(t40196: f64, t760: f64, t10696: f64, t73: f64, t138: f64, t785: f64, t9302: f64, t234: f64, t39545: f64, t685: f64, t875: f64, t2778: f64, t39515: f64, t39501: f64, t871: f64, t10115: f64, t225: f64, t10866: f64, t232: f64, t235: f64, t239: f64, t820: f64, t2723: f64, t2482: f64, t2719: f64, t596: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40198, t40231, t40270, t40294, t40314) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1337(t40196, t760, t10696, t73, t138, t785, t9302, t234, t39545, t685, t875, t2778, t39515);
        let (t40316, t40317, t40321, t40324, t40325, t40336) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1338(t39501, t871, t10115, t225, t10866, t232, t235, t239, t820, t2723, t2482, t2719, t596);
    (t40198, t40231, t40270, t40294, t40314, t40316, t40317, t40321, t40324, t40325, t40336)
}
