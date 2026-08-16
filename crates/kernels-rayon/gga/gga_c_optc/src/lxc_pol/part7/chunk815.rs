//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 815/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk815(t2375: f64, t7676: f64, t228: f64, t2414: f64, t216: f64, t2418: f64, t7670: f64, t2409: f64, t798: f64, t2416: f64, t799: f64, t800: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7678 = 6.0_f64 * t7676 * t2375;
    let t7680 = 1.0_f64 / t2414 / t228;
    let t7681 = t216 * t7680;
    let t7682 = t7670 * t2418;
    let t7684 = 0.96490945932906628932e2_f64 * t7681 * t7682;
    let t7686 = t2409 * t2418 * t798;
    let t7688 = 0.48245472966453314466e2_f64 * t2416 * t7686;
    let t7689 = t7670 * t799;
    let t7691 = 6.0_f64 * t2416 * t7689;
    let t7692 = t800 * t2409;
    (t7678, t7680, t7681, t7682, t7684, t7686, t7688, t7689, t7691, t7692)
}
