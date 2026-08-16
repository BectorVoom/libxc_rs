//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 922/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk922(t39705: f64, t8650: f64, t1502: f64, t2318: f64, t34975: f64, t34976: f64, t2281: f64, t35039: f64, t9145: f64, t16503: f64, t38508: f64, t8420: f64) -> (f64, f64, f64, f64) {
    let t45374 = t39705 * t8650;
    let t45381 = t34975 * t34976 * t2318 * t1502;
    let t45385 = t34975 * t35039 * t2281 * t9145;
    let t45389 = t16503 * t38508 * t2281 * t8420;
    (t45374, t45381, t45385, t45389)
}
