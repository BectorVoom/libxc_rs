//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 989/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk989(t2331: f64, t943: f64, t7908: f64, t8998: f64, t33489: f64, t7963: f64, t7965: f64, t4210: f64, t7942: f64, t315: f64, t5386: f64, t610: f64) -> (f64, f64, f64, f64, f64) {
    let t33597 = t2331 * t943;
    let t33606 = 0.34694512752820797848e1_f64 * t8998 * t7908;
    let t33621 = 0.17347256376410398924e1_f64 * t7963 * t33489 * t7965;
    let t33624 = 0.17347256376410398924e1_f64 * t7942 * t33489 * t4210;
    let t33627 = 0.26341796731742046394e1_f64 * t315 * t610 * t5386;
    (t33597, t33606, t33621, t33624, t33627)
}
