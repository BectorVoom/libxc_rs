//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1371/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1371<F: Float>(t1146: F, t8980: F, t3160: F, t3169: F, t1141: F, t8996: F, t469: F, t8995: F, t454: F, t3171: F, t3264: F, t1179: F, t27126: F) -> (F, F, F, F, F, F, F) {
    let t27255 = t8980 * t1146;
    let t27259 = t3160 * t3169;
    let t27266 = t1141 * t8996;
    let t27276 = F::new(1.0) / t8995 / t469;
    let t27277 = t454 * t27276;
    let t27278 = t3171 * t3171;
    let t27286 = t3264 * t3264;
    let t27297 = t1179 * t27126;
    (t27255, t27259, t27266, t27277, t27278, t27286, t27297)
}
