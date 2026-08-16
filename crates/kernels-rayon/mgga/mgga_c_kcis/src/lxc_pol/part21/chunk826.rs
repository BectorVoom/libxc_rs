//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 826/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk826(t3255: f64, t3271: f64, t3276: f64, t3250: f64, t41: f64, t85: f64, t1106: f64, t3285: f64, t3265: f64, t3296: f64, t346: f64, t9368: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10333 = t3255 * t3271;
    let t10335 = t3255 * t3276;
    let t10338 = t85 * t3250 * t41;
    let t10339 = t10338 * t1106;
    let t10341 = t3255 * t3285;
    let t10343 = t3255 * t3265;
    let t10351 = t3255 * t3296;
    let t10386 = t9368 * t346;
    (t10333, t10335, t10338, t10339, t10341, t10343, t10351, t10386)
}
