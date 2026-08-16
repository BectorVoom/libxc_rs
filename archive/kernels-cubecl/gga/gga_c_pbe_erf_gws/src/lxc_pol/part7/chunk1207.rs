//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1207/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1207<F: Float>(t20618: F, t2157: F, t2289: F, t6234: F, t2142: F, t6621: F, t2319: F, t6262: F, t2080: F, t20807: F, t2083: F, t2085: F, t860: F) -> (F, F, F, F, F) {
    let t21447 = t20618 * t2157;
    let t21452 = t2289 * t6234;
    let t21454 = t6621 * t2142;
    let t21455 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t21454;
    let t21456 = t2319 * t6262;
    let t21462 = t2080 * t20807 * t2083 * t2085 * t860 / F::cast_from(32.0_f64);
    (t21447, t21452, t21455, t21456, t21462)
}
