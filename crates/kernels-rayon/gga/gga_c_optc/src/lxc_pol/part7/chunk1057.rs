//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1057/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1057(t2024: f64, t2067: f64, t9870: f64, t2120: f64, t7026: f64, t22245: f64, t5: f64, t127: f64, t2034: f64, t2126: f64, t2168: f64, t22188: f64, t22193: f64, t22197: f64, t22198: f64, t22218: f64, t22225: f64, t22247: f64, t22876: f64, t22884: f64, t22949: f64, t3467: f64, t3501: f64, t673: f64, t675: f64, t695: f64, t696: f64, t7129: f64, t9955: f64) -> (f64, f64, f64) {
    let t22969 = t2067 * t2024;
    let t22970 = t22969 * t9870;
    let t22977 = t2120 * t7026;
    let t22979 = t5 * t22245;
    let t22984 = 0.10156750018806222173e2_f64 * t22949 + 0.36274107210022222046e0_f64 * t2168 * t22876 - 0.21764464326013333228e1_f64 * t3501 * t22198 + 0.62590762726479056551e1_f64 * t3467 * t7129 * t22218 - 0.417271751509860377e1_f64 * t3467 * t2126 * t22197 - 0.10882232163006666614e1_f64 * t9955 * t22193 + 0.81616741222549999602e0_f64 * t3501 * t22884 + 0.36274107210022222046e0_f64 * t2168 * t22225 + 0.29019285768017777637e1_f64 * t9955 * t22188 - 0.21764464326013333228e1_f64 * t3501 * t2034 * t22970 - 0.15114211337509259186e-1_f64 * t695 * t696 * t22247 + 0.81136173904695073308e0_f64 * t22977 - 0.86931614897887578546e-1_f64 * t673 * t675 * t22979 * t127;
    (t22970, t22979, t22984)
}
