//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1001/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1001(t2508: f64, t8503: f64, t9739: f64, t28953: f64, t9014: f64, t1897: f64, t2580: f64, t28236: f64, t2958: f64, t40775: f64, t1022: f64, t6058: f64) -> (f64, f64, f64, f64, f64) {
    let t43182 = 0.38452631150711121418e0_f64 * t2508 * t9739 * t8503;
    let t43185 = 0.18457262952341338281e0_f64 * t2508 * t9014 * t28953;
    let t43189 = 0.15381052460284448567e-1_f64 * t1897 * t2580 * t2958 * t28236;
    let t43190 = 0.1922631557535556071e-2_f64 * t40775;
    let t43191 = t6058 * t1022;
    (t43182, t43185, t43189, t43190, t43191)
}
