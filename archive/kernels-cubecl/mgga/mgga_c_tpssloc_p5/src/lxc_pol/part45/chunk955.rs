//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 955/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk955<F: Float>(t114121: F, t1351: F, t1992: F, t550: F, t6955: F, t6976: F, t31091: F, t80650: F, t22633: F, t31100: F, t1985: F, t22666: F, t31123: F) -> (F, F, F, F, F) {
    let t114122 = F::cast_from(0.16449340668482264365e-1_f64) * t114121;
    let t114127 = F::cast_from(0.3289868133696452873e-1_f64) * t1992 * t6976 * t6955 * t1351 * t550;
    let t114140 = F::cast_from(0.6579736267392905746e-1_f64) * t1992 * t80650 * t31091;
    let t114145 = F::cast_from(0.6579736267392905746e-1_f64) * t22633 * t80650 * t31100;
    let t114150 = F::cast_from(0.3289868133696452873e-1_f64) * t1985 * t22666 * t31123;
    (t114122, t114127, t114140, t114145, t114150)
}
