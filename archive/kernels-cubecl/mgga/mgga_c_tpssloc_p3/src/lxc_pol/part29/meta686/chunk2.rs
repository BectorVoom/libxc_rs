//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2352/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2352<F: Float>(t1851: F, t7426: F, t27907: F, t580: F, t2169: F, t5381: F, t16507: F, t16546: F, t2170: F, t2174: F, t3: F, t3932: F, t3946: F, t5364: F, t7416: F, t8111: F, t8119: F, t85405: F, t96277: F, t96281: F, t96283: F) -> F {
    let t96285 = F::cast_from(2.0_f64) * t1851 * t7426;
    let t96289 = F::cast_from(2.0_f64) * t27907 * t580;
    let t96291 = F::cast_from(2.0_f64) * t2169 * t5381;
    let t96297 = t3 * t580 * t96277 + t16507 * t2174 + t16546 * t2170 + t3932 * t8119 + t3946 * t8111 + F::cast_from(2.0_f64) * t5364 * t7426 + F::cast_from(2.0_f64) * t5381 * t7416 + F::cast_from(2.0_f64) * t85405 + t96281 + t96283 + t96285 + t96289 + t96291;
    t96297
}
