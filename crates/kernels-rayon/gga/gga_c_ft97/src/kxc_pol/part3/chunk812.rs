//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 812/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk812(t1526: f64, t4641: f64, t7705: f64, t142: f64, t8633: f64, t2984: f64, t2258: f64, t2993: f64, t18: f64, t1943: f64, t342: f64, t4645: f64, t630: f64) -> (f64, f64, f64, f64, f64) {
    let t16631 = t1526 * t7705 * t4641;
    let t16633 = t8633 * t142;
    let t16634 = t16633 * t2984;
    let t16640 = t2258 * t142;
    let t16641 = t16640 * t2993;
    let t16644 = t1943 * t18;
    let t16649 = t342 * t630 * t4645;
    (t16631, t16634, t16641, t16644, t16649)
}
