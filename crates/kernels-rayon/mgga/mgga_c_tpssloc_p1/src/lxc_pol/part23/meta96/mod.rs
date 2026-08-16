//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta96 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk538;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk539;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk540;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk541;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta96(t3355: f64, t427: f64, t435: f64, t3236: f64, t1146: f64, t445: f64, t440: f64, t3293: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3356, t3357, t3358, t3359) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk538(t3355, t427, t435);
        let (t3363, t3374, t3375) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk539(t3236, t1146, t445);
        let (t3376, t3383, t3390, t3399) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk540(t3375, t440, t3236, t3293, t1146);
        let t3400 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk541(t3399);
    (t3356, t3357, t3358, t3359, t3363, t3374, t3375, t3376, t3383, t3390, t3399, t3400)
}
