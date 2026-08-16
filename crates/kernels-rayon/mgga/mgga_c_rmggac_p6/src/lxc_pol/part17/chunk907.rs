//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 907/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk907(t16504: f64, t34975: f64, t552: f64, t9145: f64, t14237: f64, t16503: f64, t2281: f64, t8430: f64, t35039: f64, t38523: f64, t8435: f64, t8368: f64, t8568: f64) -> (f64, f64, f64, f64) {
    let t45197 = t34975 * t16504 * t552 * t9145;
    let t45201 = t16503 * t14237 * t2281 * t8430;
    let t45205 = t16503 * t35039 * t38523 * t8435;
    let t45207 = t8368 * t8568;
    (t45197, t45201, t45205, t45207)
}
