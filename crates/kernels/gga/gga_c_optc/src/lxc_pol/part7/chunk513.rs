//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 513/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk513<F: Float>(t960: F, t339: F, t2341: F, t285: F, t854: F, t858: F, t116: F, t2350: F, t286: F, t309: F, t884: F, t300: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2566 = t960 * t960;
    let t2568 = t339 * t339;
    let t2569 = F::cast_from(1.0_f64) / t2568;
    let t2573 = sigma0 * t2341;
    let t2574 = t2573 * t285;
    let t2577 = t854 * t858;
    let t2579 = t116 * t2350;
    let t2581 = t286 * t2579 / F::cast_from(432.0_f64);
    let t2582 = t884 * t309;
    let t2583 = t300 * t2582;
    (t2566, t2568, t2569, t2573, t2574, t2577, t2579, t2581, t2582, t2583)
}
