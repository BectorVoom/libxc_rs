//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1201/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1201(t2157: f64, t2720: f64, t9194: f64, t2398: f64, t8939: f64, t26459: f64, t7639: f64, t36533: f64, t695: f64, t26477: f64, t7642: f64, t209: f64, t213: f64, t36902: f64, t8762: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92002 = t2720 * t9194 * t2157;
    let t92005 = t8939 * t2398 * t2157;
    let t92007 = t26459 * t7639;
    let t92010 = t36533 * t695 * t7639;
    let t92012 = t7642 * t26477;
    let t92016 = t209 * t213 * t36902 * t8762;
    (t92002, t92005, t92007, t92010, t92012, t92016)
}
