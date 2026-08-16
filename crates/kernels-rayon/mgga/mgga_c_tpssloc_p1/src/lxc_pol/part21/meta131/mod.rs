//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta131 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk874;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk875;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk876;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk877;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk878;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk879;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta131(t1065: f64, t3174: f64, t1057: f64, t3112: f64, t3032: f64, t3127: f64, t3031: f64, t3040: f64, t381: f64, t1932: f64, t3131: f64, t1022: f64, t1049: f64, t1060: f64, t3120: f64, t1014: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3175, t3176, t3180) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk874(t1065, t3174, t1057, t3112);
        let t3185 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk875(t3032, t3127);
        let t3186 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk876(t3031, t3185);
        let (t3187, t3188) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk877(t3040, t381, t1932, t3131);
        let (t3189, t3193, t3197, t3199) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk878(t3187, t3188, t1022, t1049, t1060, t3120, t381, t1014, t3032);
        let t3200 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk879(t3031, t3199);
    (t3175, t3176, t3180, t3185, t3186, t3187, t3188, t3189, t3193, t3197, t3199, t3200)
}
