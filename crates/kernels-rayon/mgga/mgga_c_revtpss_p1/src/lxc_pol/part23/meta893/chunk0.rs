//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2849/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2849(t61093: f64, t50903: f64, t6002: f64, t14613: f64, t18539: f64, t18544: f64, t4311: f64, t23214: f64, t750: f64, t49897: f64, t14386: f64, t5999: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t76944 = 0.54934341918019635162e-3_f64 * t61093;
    let t76946 = 36.0_f64 * t50903 * t6002;
    let t76947 = t14613 * t18539;
    let t76948 = 36.0_f64 * t76947;
    let t76949 = t4311 * t18544;
    let t76950 = 12.0_f64 * t76949;
    let t76951 = t23214 * t750;
    let t76952 = 0.17544670867903938621e1_f64 * t49897;
    let t76954 = 12.0_f64 * t14386 * t5999;
    (t76944, t76946, t76948, t76950, t76951, t76952, t76954)
}
