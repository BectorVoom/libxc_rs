//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1130/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1130(t27803: f64, t7703: f64, t291: f64, t417: f64, t1008: f64, t13097: f64) -> (f64, f64, f64, f64) {
    let t27804 = t7703 * t27803;
    let t27806 = t417 * t291;
    let t27807 = t13097 * t1008;
    let t27808 = t27806 * t27807;
    (t27804, t27806, t27807, t27808)
}
