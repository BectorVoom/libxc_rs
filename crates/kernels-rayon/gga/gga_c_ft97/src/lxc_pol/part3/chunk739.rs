//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 739/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk739(t11: f64, t340: f64, t14: f64, t15564: f64, t81: f64, t8633: f64, t2984: f64, t2258: f64, t2993: f64, t1528: f64, t18: f64, t342: f64, t4410: f64, t630: f64) -> (f64, f64, f64, f64, f64) {
    let t15565 = t340 * t11;
    let t15567 = t15564 * t15565 * t14;
    let t15568 = t8633 * t81;
    let t15569 = t15568 * t2984;
    let t15575 = t2258 * t81;
    let t15576 = t15575 * t2993;
    let t15579 = t1528 * t18;
    let t15584 = t342 * t630 * t4410;
    (t15567, t15569, t15576, t15579, t15584)
}
