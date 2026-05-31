//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1209/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1209<F: Float>(t1452: F, t810: F, t4422: F, t885: F, t2149: F, t2146: F, t6406: F, t6416: F, t6158: F, t6670: F, t822: F, t20480: F, t3065: F, t858: F) -> (F, F, F, F, F) {
    let t21482 = t1452 * t810;
    let t21491 = t4422 * t885;
    let t21492 = t21491 * t2149;
    let t21493 = t2146 * t21492;
    let t21494 = F::cast_from(35.0_f64) / F::cast_from(18.0_f64) * t21493;
    let t21495 = t6416 * t6406;
    let t21497 = t6158 * t6670;
    let t21498 = t822 * t21497;
    let t21500 = t3065 * t858 * t20480;
    (t21482, t21494, t21495, t21498, t21500)
}
