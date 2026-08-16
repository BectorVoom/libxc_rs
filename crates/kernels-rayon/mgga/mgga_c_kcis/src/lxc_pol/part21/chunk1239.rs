//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1239/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1239(t26972: f64, t7780: f64, t26954: f64, t27013: f64, t27069: f64, t7772: f64, t92751: f64, t1250: f64, t251: f64, t34814: f64, t92945: f64, t35576: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t93016 = t7780 * t26972;
    let t93023 = t27013 * t26954;
    let t93028 = t27069 * t26954;
    let t93047 = t7772 * t92751;
    let t93050 = t34814 * t251 * t1250;
    let t93053 = t7772 * t92945;
    let t93056 = t35576 * t251 * t1250;
    (t93016, t93023, t93028, t93047, t93050, t93053, t93056)
}
