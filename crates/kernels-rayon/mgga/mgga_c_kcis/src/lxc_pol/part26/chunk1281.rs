//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1281/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1281(t23157: f64, t7977: f64, t1464: f64, t28360: f64, t98470: f64, t28382: f64, t28499: f64, t101943: f64, t7968: f64, t28721: f64, t28778: f64, t1489: f64, t28503: f64, t7282: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t101950 = t7977 * t23157;
    let t101954 = t1464 * t98470 * t28360;
    let t101957 = t1464 * t28499 * t28382;
    let t101959 = t7968 * t101943;
    let t101961 = t28721 * t28778;
    let t101965 = t1464 * t28503 * t7282 * t1489;
    (t101950, t101954, t101957, t101959, t101961, t101965)
}
