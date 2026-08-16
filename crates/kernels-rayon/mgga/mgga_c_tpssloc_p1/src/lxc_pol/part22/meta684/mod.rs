//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta684 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2250;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2251;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2252;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2253;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2254;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2255;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta684(t1036: f64, t18010: f64, t14025: f64, t14508: f64, t13970: f64, t14511: f64, t1057: f64, t61729: f64, t3199: f64, t61734: f64, t3185: f64, t18053: f64, t225: f64, t18238: f64, t690: f64, t18233: f64, t18207: f64, t2394: f64, t5972: f64, t5980: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62893, t62901, t62903, t62994, t63004, t63183, t63215) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2250(t1036, t18010, t14025, t14508, t13970, t14511, t1057, t61729, t3199, t61734, t3185, t18053, t225);
        let t63291 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2251(t18238, t690);
        let t63306 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2252(t18233, t690);
        let t63308 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2253(t18207, t690);
        let t63332 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2254(t2394, t5972);
        let t63334 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2255(t2394, t5980);
    (t62893, t62901, t62903, t62994, t63004, t63183, t63215, t63291, t63306, t63308, t63332, t63334)
}
