//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1071/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1071<F: Float>(t20618: F, t2157: F, t2289: F, t6234: F, t2142: F, t6621: F, t2319: F, t6262: F, t2080: F, t20807: F, t2083: F, t2085: F, t860: F, t2273: F, t6717: F, t2339: F) -> (F, F, F, F, F, F, F) {
    let t21447 = t20618 * t2157;
    let t21452 = t2289 * t6234;
    let t21454 = t6621 * t2142;
    let t21455 = 7.0 / 72.0 * t21454;
    let t21456 = t2319 * t6262;
    let t21462 = t2080 * t20807 * t2083 * t2085 * t860 / 32.0;
    let t21463 = t6717 * t2273;
    let t21465 = t6717 * t2339;
    (t21447, t21452, t21455, t21456, t21462, t21463, t21465)
}
