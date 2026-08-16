//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 438/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk438(t2347: f64, t2440: f64, t2349: f64, t420: f64, t701: f64, t2360: f64, t703: f64, t1934: f64, t704: f64, t2435: f64, t2437: f64, t695: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2441 = t2440 * t2347;
    let t2442 = t2441 * t2349;
    let t2443 = t420 * t2442;
    let t2444 = t701 * t2443;
    let t2446 = t703 * t2360;
    let t2447 = t2446 * t2349;
    let t2448 = t420 * t2447;
    let t2449 = t701 * t2448;
    let t2451 = t704 * t1934;
    let t2452 = t420 * t2451;
    let t2453 = t701 * t2452;
    let t2455 = -t2435 + 0.42562405586419753086e-2_f64 * t2437 + 0.85124811172839506173e-2_f64 * t2444 - 0.12768721675925925926e-1_f64 * t2449 + 0.6384360837962962963e-2_f64 * t2453;
    let t2456 = t695 * t2455;
    (t2442, t2443, t2444, t2447, t2448, t2449, t2451, t2452, t2453, t2455, t2456)
}
