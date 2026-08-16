//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 869/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk869(t1707: f64, t3351: f64, t498: f64, t511: f64, t7248: f64, t34659: f64, t34662: f64, t34665: f64, t38312: f64, t38315: f64, t38318: f64, t38322: f64, t38326: f64, t44580: f64, t44584: f64, t44590: f64, t44595: f64, t44600: f64, t44605: f64, t44610: f64, t44615: f64) -> f64 {
    let t44620 = t3351 * t7248 * t511 * t1707 * t498;
    let t44622 = 0.16260079888840015101e-2_f64 * t38312 + t38315 - 0.66671395154821946449e-1_f64 * t38318 + 0.33335697577410973224e-1_f64 * t34659 - 0.3903207359137154578e-3_f64 * t38322 + 0.60975299583150056628e-3_f64 * t38326 + 0.14905073231436680509e-2_f64 * t34662 + 0.14905073231436680509e-2_f64 * t34665 - 0.42564599893297839398e-5_f64 * t44580 + 0.17025839957319135759e-4_f64 * t44584 + 0.85129199786595678796e-5_f64 * t44590 + 0.53205749866622299248e-5_f64 * t44595 - 0.85129199786595678796e-5_f64 * t44600 + 0.25538759935978703639e-4_f64 * t44605 - 0.25538759935978703639e-4_f64 * t44610 - 0.85129199786595678796e-5_f64 * t44615 + 0.76616279807936110914e-4_f64 * t44620;
    t44622
}
