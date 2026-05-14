//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 651/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk651<F: Float>(t703: F, t713: F, t712: F, t247: F, t4562: F, t251: F, t1906: F, t719: F, t256: F, t1354: F, t19: F, t336: F, t714: F, t1791: F, t1793: F, t617: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5441 = t703 * t713;
    let t5443 = 0.13506172839506172839e-1 * t712 * t5441;
    let t5444 = t4562 * t247;
    let t5445 = t5444 * t251;
    let t5448 = t1906 * t719;
    let t5449 = t5448 * t256;
    let t5450 = t1354 * t19;
    let t5451 = t5450 * t336;
    let t5452 = t5451 * t714;
    let t5454 = t1791 * t1793;
    let t5455 = t5454 * t617;
    (t5441, t5443, t5444, t5445, t5448, t5449, t5450, t5451, t5452, t5454, t5455)
}
