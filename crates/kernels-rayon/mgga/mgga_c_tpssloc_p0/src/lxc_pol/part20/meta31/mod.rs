//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta31 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk227;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk228;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk229;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk230;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk231;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk232;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk233;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta31(t19: f64, t598: f64, t582: f64, t586: f64, t589: f64, t593: f64, t596: f64, t83: f64, t85: f64, t24: f64, t4: f64, t581: f64, t25: f64, t28: f64, zeta_threshold: f64, t31: f64, t65: f64, t34: f64, t36: f64, rho0: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t600, t601, t604) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk227(t19, t598, t582, t586, t589, t593, t596, t83, t85);
        let t605 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk228(t24, t604);
        let t606 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk229(t4, t581);
        let t607 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk230(t25, t28, t606, zeta_threshold);
        let t608 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk231(t31, t607);
        let t609 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk232(t608, t65);
        let (t612, t614, t615) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk233(t34, t36, rho0, sigma0);
    (t600, t601, t604, t605, t606, t607, t608, t609, t612, t614, t615)
}
