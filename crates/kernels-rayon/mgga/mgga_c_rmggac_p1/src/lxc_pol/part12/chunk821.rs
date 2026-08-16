//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 821/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk821(t209: f64, t333: f64, t16503: f64, t3369: f64, t352: f64, t38422: f64, t34761: f64, t8432: f64, t205: f64, t24985: f64, t3350: f64, t671: f64) -> (f64, f64, f64, f64) {
    let t38444 = t209 * t333;
    let t38448 = t16503 * t3369 * t38422 * t38444 * t352;
    let t38450 = t34761 * t8432;
    let t38454 = t671 * t24985 * t205 * t3350;
    (t38444, t38448, t38450, t38454)
}
