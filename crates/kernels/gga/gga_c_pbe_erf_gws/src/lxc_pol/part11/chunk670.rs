//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 670/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk670<F: Float>(t3351: F, t5002: F, t5063: F, t3466: F, t395: F, t3470: F, t3474: F, t3583: F, t719: F, t256: F, t19: F, t3379: F, t336: F, t714: F, t1033: F, t2749: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10523 = t5002 * t3351;
    let t10534 = t5063 * t3351;
    let t10581 = t395 * t3466;
    let t10583 = t395 * t3470;
    let t10585 = t395 * t3474;
    let t10606 = t3583 * t719;
    let t10607 = t10606 * t256;
    let t10609 = t3379 * t19;
    let t10610 = t10609 * t336;
    let t10611 = t10610 * t714;
    let t10617 = t1033 * t2749;
    (t10523, t10534, t10581, t10583, t10585, t10606, t10607, t10609, t10610, t10611, t10617)
}
