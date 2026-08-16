//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 222/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk222(t931: f64, t932: f64, t880: f64, t886: f64, t324: f64, t320: f64, t315: f64, t906: f64, t897: f64, t902: f64, t910: f64, t323: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t933 = t931 * t932;
    let t936 = 0.92708333333333333333e-2_f64 * t880;
    let t938 = -t936 - 0.92708333333333333333e-2_f64 * t886;
    let t939 = t938 * t324;
    let t941 = t320 * t320;
    let t942 = 1.0_f64 / t941;
    let t943 = t315 * t942;
    let t945 = 0.301925e0_f64 * t880;
    let t948 = 0.82785e-1_f64 * t906;
    let t950 = 0.258925e1_f64 * t897 - t945 - 0.301925e0_f64 * t886 + 0.16504875e0_f64 * t902 - t948 - 0.82785e-1_f64 * t910;
    let t951 = 1.0_f64 / t323;
    (t933, t938, t939, t941, t942, t943, t950, t951)
}
