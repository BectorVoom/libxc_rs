//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1387/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1387<F: Float>(t1162: F, t3088: F, t7274: F, t3097: F, t1179: F, t27004: F, t8470: F, t9170: F, t3181: F, t442: F, t462: F, t27173: F) -> (F, F, F, F, F) {
    let t27616 = t1162 * t7274 * t3088;
    let t27619 = t1162 * t7274 * t3097;
    let t27621 = t1179 * t27004;
    let t27623 = t9170 * t8470;
    let t27629 = F::cast_from(1.0_f64) / t3181 / t462 * t442;
    let t27630 = t27629 * t27173;
    (t27616, t27619, t27621, t27623, t27630)
}
