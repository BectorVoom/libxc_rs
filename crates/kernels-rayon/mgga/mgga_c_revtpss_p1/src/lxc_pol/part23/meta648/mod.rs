//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta648 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2372;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2373;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta648(t138: f64, t785: f64, t9302: f64, t2786: f64, t234: f64, t39545: f64, t685: f64, t875: f64, t2778: f64, t39515: f64, t39501: f64, t871: f64, t10115: f64, t225: f64, t880: f64, t10866: f64, t232: f64, t235: f64, t2723: f64, t2482: f64, t2719: f64, t596: f64, t10868: f64, t820: f64, t843: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40270, t40271, t40294, t40314, t40316) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2372(t138, t785, t9302, t2786, t234, t39545, t685, t875, t2778, t39515, t39501, t871);
        let (t40317, t40318, t40321, t40322, t40325, t40336, t40348) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2373(t10115, t225, t880, t10866, t232, t235, t2723, t2482, t2719, t596, t10868, t820, t843);
    (t40270, t40271, t40294, t40314, t40316, t40317, t40318, t40321, t40322, t40325, t40336, t40348)
}
