//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 506/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk506(t1736: f64, t70: f64, t2252: f64, t342: f64, t511: f64, t1526: f64, t1944: f64, t7705: f64, t1948: f64, t630: f64, t128: f64, t39: f64) -> (f64, f64, f64, f64, f64) {
    let t8633 = t70 * t1736;
    let t8759 = t342 * t2252 * t511 / 18.0_f64;
    let t8761 = t1526 * t7705 * t1944;
    let t8764 = t342 * t630 * t1948;
    let t8811 = t128 * t39;
    (t8633, t8759, t8761, t8764, t8811)
}
