//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta90 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk646;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk647;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk648;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk649;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk650;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta90(t2250: f64, t31: f64, t65: f64, t608: f64, t628: f64, t36: f64, t365: f64, sigma0: f64, t42: f64, t2244: f64, t43: f64, t54: f64, t55: f64, t240: f64, t59: f64, t39: f64, t44: f64, t51: f64, t615: f64, t618: f64, t33: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2251, t2252, t2255, t2262) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk646(t2250, t31, t65, t608, t628, t36, t365, sigma0);
        let t2267 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk647(t42);
        let (t2268, t2271, t2274) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk648(t2244, t2267, t2250, t43, t54);
        let (t2275, t2278, t2281) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk649(t2244, t2274, t2250, t55, t240, t59);
        let (t2282, t2283, t2284) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk650(t2281, t2262, t2268, t2271, t2275, t2278, t39, t44, t51, t615, t618, t33);
    (t2251, t2252, t2255, t2262, t2267, t2274, t2275, t2278, t2281, t2282, t2283, t2284)
}
