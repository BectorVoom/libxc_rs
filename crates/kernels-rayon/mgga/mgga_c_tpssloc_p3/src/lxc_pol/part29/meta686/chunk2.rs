//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2352/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2352(t1851: f64, t7426: f64, t27907: f64, t580: f64, t2169: f64, t5381: f64, t16507: f64, t16546: f64, t2170: f64, t2174: f64, t3: f64, t3932: f64, t3946: f64, t5364: f64, t7416: f64, t8111: f64, t8119: f64, t85405: f64, t96277: f64, t96281: f64, t96283: f64) -> f64 {
    let t96285 = 2.0_f64 * t1851 * t7426;
    let t96289 = 2.0_f64 * t27907 * t580;
    let t96291 = 2.0_f64 * t2169 * t5381;
    let t96297 = t3 * t580 * t96277 + t16507 * t2174 + t16546 * t2170 + t3932 * t8119 + t3946 * t8111 + 2.0_f64 * t5364 * t7426 + 2.0_f64 * t5381 * t7416 + 2.0_f64 * t85405 + t96281 + t96283 + t96285 + t96289 + t96291;
    t96297
}
