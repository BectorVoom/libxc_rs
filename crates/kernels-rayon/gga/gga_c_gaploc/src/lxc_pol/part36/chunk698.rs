//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 698/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk698(t12456: f64, t12906: f64, t12909: f64, t12911: f64, t12912: f64, t12916: f64, t12921: f64, t12924: f64, t12928: f64, t12929: f64, t12930: f64, t12931: f64) -> f64 {
    let t12932 = 0.59584149919750711116e-1_f64 * t12456;
    let t12933 = -0.92023022289409799224e1_f64 * t12906 + t12909 + t12911 + 0.71500979903700853338e0_f64 * t12912 - 0.13803453343411469884e2_f64 * t12916 - t12921 + t12924 - t12928 - t12929 + t12930 - t12931 + t12932;
    t12933
}
