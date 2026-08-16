//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta637 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2424;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2425;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2426;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2427;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta637(t2394: f64, t2772: f64, t10969: f64, t154: f64, t2769: f64, t2777: f64, t885: f64, t9698: f64, t2289: f64, t41654: f64, t2784: f64, t2791: f64, t2897: f64, t2929: f64, t10629: f64, t938: f64, t2903: f64, t2928: f64, t315: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t41658 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2424(t2394, t2772);
        let (t41664, t41666, t41675) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2425(t10969, t154, t2769, t2394, t2777);
        let t41684 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2426(t885, t9698);
        let (t41687, t41741, t41811, t41816, t41821, t41825, t41826) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2427(t2289, t2769, t41654, t2784, t2791, t2897, t2929, t10629, t938, t2903, t2928, t315);
    (t41658, t41664, t41666, t41675, t41684, t41687, t41741, t41811, t41816, t41821, t41825, t41826)
}
