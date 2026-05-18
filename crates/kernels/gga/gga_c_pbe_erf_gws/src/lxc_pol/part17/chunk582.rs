//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 582/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk582<F: Float>(t1022: F, t626: F, t422: F, t1809: F, t1620: F, t1027: F, t617: F, t572: F, t995: F, t418: F, t1821: F, t1820: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2570 = t1022 * t626;
    let t2571 = t2570 * t422;
    let t2572 = t1809 * t2571;
    let t2574 = F::new(8.0) / F::new(45.0) * t1620 * t2572;
    let t2575 = t1027 * t617;
    let t2576 = t1809 * t2575;
    let t2578 = F::new(8.0) / F::new(45.0) * t1620 * t2576;
    let t2579 = t995 * t572;
    let t2580 = t2579 * t418;
    let t2581 = t1821 * t2580;
    let t2583 = F::new(8.0) / F::new(45.0) * t1820 * t2581;
    (t2570, t2571, t2572, t2574, t2575, t2576, t2578, t2579, t2580, t2581, t2583)
}
