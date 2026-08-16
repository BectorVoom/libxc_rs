//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1927/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1927(t1937: f64, t25628: f64, t1618: f64, t1622: f64, t1935: f64, t23433: f64, t23443: f64, t23447: f64, t23449: f64, t23463: f64, t23469: f64, t23529: f64, t25609: f64, t25616: f64, t25618: f64, t25622: f64, t25625: f64, t378: f64, t6730: f64, t7578: f64) -> f64 {
    let t25629 = t25628 * t1937;
    let t25631 = -0.10093189023535097714e-3_f64 * t6730 * t7578 - 0.10093189023535097714e-3_f64 * t1935 * t25609 + 0.10093189023535097714e-3_f64 * t23443 - t23447 - 0.80745512188280781712e-3_f64 * t23449 - t23529 * t1622 / 432.0_f64 + t25616 / 3456.0_f64 + t25618 / 2304.0_f64 + t23433 * t1618 / 1536.0_f64 - t25622 * t378 / 288.0_f64 + t25625 / 2304.0_f64 - t23463 / 108.0_f64 + 0.10093189023535097714e-3_f64 * t25629 - t23469;
    t25631
}
