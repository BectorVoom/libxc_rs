//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2139/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2139(t11274: f64, t1657: f64, t50826: f64, t50853: f64, t3263: f64, t4737: f64, t11189: f64, t50919: f64, t50948: f64, t51039: f64, t51051: f64, t3400: f64, t4832: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t51120 = t1657 * t11274;
    let t51137 = 0.39862222222222222223e0_f64 * t50826;
    let t51151 = 0.27385555555555555556e0_f64 * t50853;
    let t51246 = t4737 * t3263;
    let t51249 = t1657 * t11189;
    let t51257 = 0.68863333333333333332e0_f64 * t50826;
    let t51271 = 0.34731666666666666667e0_f64 * t50853;
    let t51299 = 0.45908888888888888888e0_f64 * t50919;
    let t51310 = 0.13772666666666666666e1_f64 * t50948;
    let t51349 = 0.69463333333333333334e0_f64 * t51039;
    let t51354 = 0.11577222222222222222e0_f64 * t51051;
    let t51371 = t4832 * t3400;
    (t51120, t51137, t51151, t51246, t51249, t51257, t51271, t51299, t51310, t51349, t51354, t51371)
}
