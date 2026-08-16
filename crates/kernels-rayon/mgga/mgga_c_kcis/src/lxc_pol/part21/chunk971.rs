//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 971/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk971(t14590: f64, t3338: f64, t3337: f64, t14092: f64, t5047: f64, t5046: f64, t1133: f64, t4984: f64, t5181: f64, t3437: f64, t1797: f64, t3362: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14700 = t3338 * t14590;
    let t14701 = t3337 * t14700;
    let t14703 = t5047 * t14092;
    let t14704 = t5046 * t14703;
    let t14706 = t4984 * t1133;
    let t14707 = t5181 * t14706;
    let t14708 = t3437 * t14707;
    let t14710 = t1797 * t3362;
    (t14700, t14701, t14703, t14704, t14706, t14707, t14708, t14710)
}
