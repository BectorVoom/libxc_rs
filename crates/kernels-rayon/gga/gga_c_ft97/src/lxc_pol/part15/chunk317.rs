//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 317/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk317(t358: f64, t487: f64, t342: f64, t511: f64, t630: f64, t142: f64, t10: f64, t144: f64, t1542: f64, t143: f64, t1557: f64, t378: f64, t525: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1910 = t487 * t358;
    let t1942 = t342 * t630 * t511 / 12.0_f64;
    let t1943 = t142 * t358;
    let t1956 = t10 * t1542 * t144;
    let t1957 = 2.0_f64 / 27.0_f64 * t1956;
    let t1964 = t143 * t1557;
    let t1969 = t378 * t525;
    (t1910, t1942, t1943, t1956, t1957, t1964, t1969)
}
