//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1057/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1057<F: Float>(t2024: F, t2067: F, t9870: F, t2120: F, t7026: F, t22245: F, t5: F, t127: F, t2034: F, t2126: F, t2168: F, t22188: F, t22193: F, t22197: F, t22198: F, t22218: F, t22225: F, t22247: F, t22876: F, t22884: F, t22949: F, t3467: F, t3501: F, t673: F, t675: F, t695: F, t696: F, t7129: F, t9955: F) -> (F, F, F) {
    let t22969 = t2067 * t2024;
    let t22970 = t22969 * t9870;
    let t22977 = t2120 * t7026;
    let t22979 = t5 * t22245;
    let t22984 = F::cast_from(0.10156750018806222173e2_f64) * t22949 + F::cast_from(0.36274107210022222046e0_f64) * t2168 * t22876 - F::cast_from(0.21764464326013333228e1_f64) * t3501 * t22198 + F::cast_from(0.62590762726479056551e1_f64) * t3467 * t7129 * t22218 - F::cast_from(0.417271751509860377e1_f64) * t3467 * t2126 * t22197 - F::cast_from(0.10882232163006666614e1_f64) * t9955 * t22193 + F::cast_from(0.81616741222549999602e0_f64) * t3501 * t22884 + F::cast_from(0.36274107210022222046e0_f64) * t2168 * t22225 + F::cast_from(0.29019285768017777637e1_f64) * t9955 * t22188 - F::cast_from(0.21764464326013333228e1_f64) * t3501 * t2034 * t22970 - F::cast_from(0.15114211337509259186e-1_f64) * t695 * t696 * t22247 + F::cast_from(0.81136173904695073308e0_f64) * t22977 - F::cast_from(0.86931614897887578546e-1_f64) * t673 * t675 * t22979 * t127;
    (t22970, t22979, t22984)
}
