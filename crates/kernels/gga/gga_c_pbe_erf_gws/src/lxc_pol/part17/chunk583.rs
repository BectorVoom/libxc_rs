//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 583/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk583<F: Float>(t1000: F, t562: F, t1821: F, t1820: F, t1037: F, t1627: F, t331: F, t641: F, t34: F, t643: F, t639: F, t1044: F, t649: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2584 = t1000 * t562;
    let t2585 = t1821 * t2584;
    let t2587 = F::new(8.0) / F::new(45.0) * t1820 * t2585;
    let t2590 = F::new(4.0) / F::new(45.0) * t1627 * t1037;
    let t2591 = t331 * t641;
    let t2592 = t643 * t34;
    let t2593 = t2591 * t2592;
    let t2595 = F::new(8.0) / F::new(45.0) * t639 * t2593;
    let t2596 = t649 * t1044;
    (t2584, t2585, t2587, t2590, t2591, t2592, t2593, t2595, t2596)
}
