//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1532/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1532(t16551: f64, t342: f64, t11631: f64, t12050: f64, t3151: f64, t15907: f64, t12077: f64, t378: f64, t3154: f64, t12046: f64, t357: f64, t3133: f64, t3302: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16552 = t342 * t16551;
    let t16553 = t12050 * t11631;
    let t16554 = t16553 * t3151;
    let t16555 = t15907 * t16554;
    let t16558 = t12077 * t378;
    let t16559 = t342 * t16558;
    let t16560 = t12050 * t3154;
    let t16561 = t16560 * t3151;
    let t16562 = t15907 * t16561;
    let t16565 = t12046 * t378;
    let t16566 = t342 * t16565;
    let t16568 = t12050 * t3151 * t357;
    let t16569 = t15907 * t16568;
    let t16573 = t3302 * t3133 * t357;
    (t16552, t16555, t16559, t16562, t16566, t16569, t16573)
}
