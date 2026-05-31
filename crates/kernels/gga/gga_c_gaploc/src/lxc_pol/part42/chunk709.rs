//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 709/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk709<F: Float>(t12693: F, t12706: F, t12223: F, t2562: F, t883: F, t943: F, t2558: F, t3732: F, t12405: F, t12784: F, t13288: F, t13291: F, t13292: F, t13293: F, t13294: F, t13295: F) -> (F, F, F, F, F, F, F) {
    let t13898 = F::cast_from(0.63904876589867916128e-1_f64) * t12693;
    let t13899 = F::cast_from(0.63904876589867916128e-1_f64) * t12706;
    let t13934 = t2562 * t883 * t12223;
    let t13935 = t943 * t13934;
    let t13937 = t3732 * t2558;
    let t13938 = t943 * t13937;
    let t14266 = t13288 + F::cast_from(2.0_f64) * t12784 - F::cast_from(2.0_f64) * t12405 - t13291 - t13292 + t13293 + t13294 + t13295;
    (t13898, t13899, t13934, t13935, t13937, t13938, t14266)
}
