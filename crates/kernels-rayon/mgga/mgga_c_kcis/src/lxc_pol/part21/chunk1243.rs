//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1243/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1243(t10466: f64, t283: f64, t990: f64, t3049: f64, t982: f64, t26748: f64, t26757: f64, t14443: f64, t26766: f64, t7703: f64, t14447: f64, t26696: f64) -> (f64, f64, f64, f64, f64) {
    let t93366 = t10466 * t283 * t990;
    let t93394 = t3049 * t982 * t990;
    let t93403 = t26748 * t26757;
    let t93406 = t7703 * t14443 * t26766;
    let t93409 = t7703 * t14447 * t26696;
    (t93366, t93394, t93403, t93406, t93409)
}
