//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2151/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2151(t25097: f64, t81782: f64, t81783: f64, t1516: f64, t81769: f64, t23133: f64, t4261: f64, t25111: f64, t25115: f64, t87229: f64, t23132: f64, t4166: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87328 = t81782 * t81783 * t25097;
    let t87329 = 0.40372756094140390854e-3_f64 * t87328;
    let t87330 = t81769 * t1516;
    let t87331 = 7.0_f64 / 288.0_f64 * t87330;
    let t87332 = t23133 * t4261;
    let t87333 = 7.0_f64 / 288.0_f64 * t87332;
    let t87335 = t81782 * t81783 * t25111;
    let t87336 = 0.40372756094140390854e-3_f64 * t87335;
    let t87338 = t87229 * t81783 * t25115;
    let t87339 = 0.6728792682356731809e-4_f64 * t87338;
    let t87340 = t4166 * t23132;
    (t87329, t87331, t87333, t87336, t87339, t87340)
}
