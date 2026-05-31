//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 524/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk524<F: Float>(t331: F, t641: F, t34: F, t643: F, t639: F, t1044: F, t649: F, t617: F, t1621: F, t1620: F, t1791: F, t661: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2591 = t331 * t641;
    let t2592 = t643 * t34;
    let t2593 = t2591 * t2592;
    let t2595 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t639 * t2593;
    let t2596 = t649 * t1044;
    let t2597 = t2596 * t617;
    let t2598 = t1621 * t2597;
    let t2600 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1620 * t2598;
    let t2601 = t1791 * t1044;
    let t2602 = t2601 * t661;
    (t2591, t2592, t2593, t2595, t2597, t2598, t2600, t2601, t2602)
}
