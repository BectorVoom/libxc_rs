//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1208/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1208(t2333: f64, t2526: f64, t795: f64, t3262: f64, t3263: f64, t10918: f64, t11625: f64, t3275: f64, t10610: f64, t10611: f64, t11479: f64, t37483: f64, t37488: f64, t37495: f64, t37499: f64, t37503: f64, t37507: f64, t37524: f64, t37528: f64, t40479: f64, t40483: f64, t40485: f64, t40490: f64) -> (f64, f64, f64, f64) {
    let t40491 = t2333 * t2526;
    let t40492 = t40491 * t795;
    let t40495 = 3.0_f64 / 2.0_f64 * t3262 * t3263 * t40492;
    let t40502 = t3275 * t10918 * t11625;
    let t40505 = 3.0_f64 / 2.0_f64 * t10610 * t11479 * t10611;
    let t40506 = t40479 - 0.3903207359137154578e-3_f64 * t37483 - t40483 + 0.14905073231436680509e-2_f64 * t40485 + t40490 + t40495 + 0.36021158228745895953e-3_f64 * t37488 + 0.72042316457491791906e-3_f64 * t37495 - 0.51240438831339423711e-4_f64 * t37499 + 0.72042316457491791906e-3_f64 * t37503 - 0.10248087766267884742e-3_f64 * t37507 - t40502 - t40505 + t37524 - t37528;
    (t40495, t40502, t40505, t40506)
}
