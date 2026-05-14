//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 509/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk509<F: Float>(t1820: F, t2581: F, t1000: F, t562: F, t1821: F, t1037: F, t1627: F, t331: F, t641: F, t34: F, t643: F, t639: F, t1044: F, t649: F, t617: F, t1621: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2583 = 8.0 / 45.0 * t1820 * t2581;
    let t2584 = t1000 * t562;
    let t2585 = t1821 * t2584;
    let t2587 = 8.0 / 45.0 * t1820 * t2585;
    let t2590 = 4.0 / 45.0 * t1627 * t1037;
    let t2591 = t331 * t641;
    let t2592 = t643 * t34;
    let t2593 = t2591 * t2592;
    let t2595 = 8.0 / 45.0 * t639 * t2593;
    let t2596 = t649 * t1044;
    let t2597 = t2596 * t617;
    let t2598 = t1621 * t2597;
    (t2583, t2584, t2585, t2587, t2590, t2591, t2592, t2593, t2595, t2597, t2598)
}
