//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1008/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1008<F: Float>(t11506: F, t315: F, t3013: F, t323: F, t3006: F, t3014: F, t2873: F, t910: F, t11132: F, t2942: F, t941: F, t2986: F, t960: F, t2979: F, t300: F, t1034: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11507 = t315 * t11506;
    let t11509 = 1.0 / t3013 / t323;
    let t11524 = t3006 * t3014;
    let t11528 = t910 * t2873;
    let t11534 = 0.55403703703703703703e-1 * t11132;
    let t11548 = t941 * t2942;
    let t11554 = t960 * t2986;
    let t11560 = 0.28842592592592592592e-1 * t11132;
    let t11574 = 0.53272592592592592592e-1 * t11132;
    let t11591 = t300 * t2979;
    let t11626 = t1034 * t1034;
    (t11507, t11509, t11524, t11528, t11534, t11548, t11554, t11560, t11574, t11591, t11626)
}
