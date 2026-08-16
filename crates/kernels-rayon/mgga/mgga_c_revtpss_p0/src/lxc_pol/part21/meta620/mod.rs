//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta620 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2376;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2377;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta620(t10815: f64, t2648: f64, t2756: f64, t2681: f64, t2719: f64, t820: f64, t2726: f64, t10850: f64, t10861: f64, t221: f64, t2485: f64, t10111: f64, t823: f64, t9720: f64, t685: f64, t827: f64, t837: f64, t10837: f64, t9775: f64, t10828: f64, t2741: f64, t10818: f64, t10703: f64, t2674: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40393, t40395, t40399, t40403, t40406) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2376(t10815, t2648, t2756, t2681, t2719, t820, t2726, t10850, t10861, t221, t2485, t10111, t823, t9720);
        let (t40409, t40411, t40413, t40421) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2377(t40406, t685, t827, t837, t10837, t9775, t10828, t2741, t10818, t221, t10703, t2674);
    (t40393, t40395, t40399, t40403, t40406, t40409, t40411, t40413, t40421)
}
