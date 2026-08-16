//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1391/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1391(t1983: f64, t28238: f64, t5161: f64, t19596: f64, t7753: f64, t28817: f64, t7685: f64, t191: f64, t192: f64, t20350: f64, t2020: f64, t5445: f64, t72: f64, t7431: f64) -> (f64, f64, f64, f64, f64) {
    let t106744 = 3.0_f64 * t1983 * t28238 * t5161;
    let t106747 = 3.0_f64 * t1983 * t7753 * t19596;
    let t106753 = 18.0_f64 * t7685 * t28817;
    let t106755 = t20350 * t191 * t192;
    let t106756 = t106755 * t2020;
    let t106758 = t72 * t7431 * t5445;
    (t106744, t106747, t106753, t106756, t106758)
}
