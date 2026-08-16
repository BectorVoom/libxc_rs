//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 865/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk865(t30193: f64, t606: f64, t1979: f64, t980: f64, t1994: f64, t7736: f64, t993: f64, t1967: f64, t7472: f64, t1092: f64, t7614: f64, t12935: f64, t2067: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30194 = t30193 * t606;
    let t30195 = 0.15724046144802076034e-3_f64 * t30194;
    let t30196 = t980 * t1979;
    let t30197 = t30196 * t1994;
    let t30198 = 0.31448092289604152067e-3_f64 * t30197;
    let t30199 = t7736 * t993;
    let t30200 = 0.12862205435420921092e-2_f64 * t30199;
    let t30201 = t1967 * t7472;
    let t30203 = t7614 * t1092;
    let t30209 = t12935 * t2067;
    (t30195, t30196, t30198, t30200, t30201, t30203, t30209)
}
