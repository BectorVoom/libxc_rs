//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 293/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk293<F: Float>(t1212: F, t852: F, t192: F, t1228: F, t462: F, t847: F, t92: F, t845: F, t91: F, t1188: F, t1215: F, t860: F) -> (F, F, F, F) {
    let t1231 = t852 * t1212;
    let t1232 = t192 * t1231;
    let t1234 = -t847 - t462 * t1228 / F::cast_from(3.0_f64) - t92 * t1232;
    let t1236 = t91 * t845 * t1234;
    let t1240 = t1236 / F::cast_from(6.0_f64) - t860 - t1188 / F::cast_from(9.0_f64) - t1215 / F::cast_from(3.0_f64);
    (t1232, t1234, t1236, t1240)
}
