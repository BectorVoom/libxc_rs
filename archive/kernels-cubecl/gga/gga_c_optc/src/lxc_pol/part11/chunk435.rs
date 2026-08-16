//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 435/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk435<F: Float>(t2548: F, t362: F, t339: F, t116: F, t2350: F, t286: F, t141: F, t9: F) -> (F, F, F, F, F) {
    let t2549 = t2548 * t362;
    let t2568 = t339 * t339;
    let t2569 = F::cast_from(1.0_f64) / t2568;
    let t2579 = t116 * t2350;
    let t2581 = t286 * t2579 / F::cast_from(432.0_f64);
    let t2586 = t141 * t9;
    (t2549, t2568, t2569, t2581, t2586)
}
