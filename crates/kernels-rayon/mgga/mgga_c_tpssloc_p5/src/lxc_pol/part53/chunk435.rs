//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 435/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk435(t340: f64, t697: f64, t344: f64, t221: f64, t339: f64, t135: f64, t976: f64, t979: f64, t973: f64, t986: f64, t271: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2965 = t697 * t340;
    let t2966 = t2965 * t344;
    let t2967 = t221 * t2966;
    let t2969 = 0.18518518518518518518e-3_f64 * t339 * t2967;
    let t2970 = t135 * t976;
    let t2971 = t2970 * t979;
    let t2972 = t973 * t2971;
    let t2974 = t135 * t986;
    let t2975 = t973 * t2974;
    let t2978 = 1.0_f64 / t271 / t883;
    (t2965, t2969, t2970, t2972, t2975, t2978)
}
