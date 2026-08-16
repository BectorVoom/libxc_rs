//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1266/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1266(t7749: f64, t95416: f64, t28059: f64, t3339: f64, t1196: f64, t13181: f64, t8069: f64, t92540: f64, t26946: f64, t28045: f64, t26933: f64, t28050: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95417 = t95416 * t7749;
    let t95419 = t28059 * t3339;
    let t95421 = t13181 * t1196;
    let t95423 = t92540 * t8069;
    let t95425 = t28045 * t26946;
    let t95427 = t26933 * t28050;
    (t95417, t95419, t95421, t95423, t95425, t95427)
}
