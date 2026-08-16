//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta31 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk228;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk229;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk230;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk231;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk232;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta31(t592: f64, t14: f64, t2: f64, t21: f64, t15: f64, t583: f64, t19: f64, t582: f64, t586: f64, t589: f64, t83: f64, t85: f64, t24: f64, t4: f64, t581: f64, t25: f64, t28: f64, zeta_threshold: f64, t31: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t593, t594, t596, t597, t598, t600, t601, t604) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk228(t592, t14, t2, t21, t15, t583, t19, t582, t586, t589, t83, t85);
        let t605 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk229(t24, t604);
        let t606 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk230(t4, t581);
        let t607 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk231(t25, t28, t606, zeta_threshold);
        let t608 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk232(t31, t607);
    (t593, t594, t596, t597, t598, t600, t601, t604, t605, t606, t607, t608)
}
