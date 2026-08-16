//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2531/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2531(t50853: f64, t43768: f64, t43770: f64, t44249: f64, t50846: f64, t50848: f64, t50851: f64, t50859: f64, t50863: f64, t50867: f64, t50871: f64, t50875: f64) -> f64 {
    let t51271 = 0.34731666666666666667e0_f64 * t50853;
    let t51279 = -0.30872592592592592592e0_f64 * t50846 - 0.20839e0_f64 * t50848 + 0.104195e0_f64 * t50851 + t51271 + 0.69463333333333333332e-1_f64 * t43768 - 0.41678000000000000001e0_f64 * t43770 + t44249 - 0.34731666666666666667e-1_f64 * t50859 - 0.125034e1_f64 * t50863 + 0.62517e0_f64 * t50867 + 0.187551e1_f64 * t50871 + 0.20839e0_f64 * t50875;
    t51279
}
