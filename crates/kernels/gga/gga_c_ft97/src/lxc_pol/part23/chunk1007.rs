//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1007/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1007<F: Float>(t263: F, t4934: F, t24181: F, t193: F, t24232: F, t5165: F, t24231: F, t1449: F, t5147: F, t2568: F, t1168: F, t6940: F, t13927: F, t6930: F, t2354: F, t4969: F, t6003: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t30919 = t263 * t4934;
    let t30920 = t24181 * t30919;
    let t30921 = t193 * t30920;
    let t30924 = t24232 * t5165;
    let t30925 = t24231 * t30924;
    let t30930 = t1449 * t5147;
    let t30931 = t2568 * t30930;
    let t30933 = t6940 * t1168;
    let t30934 = t2568 * t30933;
    let t30936 = t13927 * t6930;
    let t30939 = t2354 * t6003 * t4969;
    (t30919, t30920, t30921, t30924, t30925, t30930, t30931, t30933, t30934, t30936, t30939)
}
