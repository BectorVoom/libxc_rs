//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 438/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk438<F: Float>(t2347: F, t2440: F, t2349: F, t420: F, t701: F, t2360: F, t703: F, t1934: F, t704: F, t2435: F, t2437: F) -> (F, F, F, F, F, F, F, F, F) {
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
    let t2455 = -t2435 + 0.42562405586419753086e-2 * t2437 + 0.85124811172839506173e-2 * t2444 - 0.12768721675925925926e-1 * t2449 + 0.6384360837962962963e-2 * t2453;
    (t2441, t2442, t2444, t2446, t2447, t2449, t2451, t2453, t2455)
}
