//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 968/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk968(t18961: f64, t3691: f64, t2917: f64, t294: f64, t3700: f64, t18: f64, t2639: f64, t342: f64, t5202: f64, t630: f64, t231: f64, t4129: f64) -> (f64, f64, f64, f64, f64) {
    let t18962 = t18961 * t3691;
    let t18968 = t2917 * t294;
    let t18969 = t18968 * t3700;
    let t18972 = t2639 * t18;
    let t18977 = t342 * t630 * t5202;
    let t18982 = t231 * t4129;
    (t18962, t18969, t18972, t18977, t18982)
}
