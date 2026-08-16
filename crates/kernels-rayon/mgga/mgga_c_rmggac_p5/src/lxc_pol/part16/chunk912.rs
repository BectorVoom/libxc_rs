//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 912/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk912(t14243: f64, t16503: f64, t552: f64, t8430: f64, t1598: f64, t16504: f64, t8435: f64, t10072: f64, t34761: f64, t1502: f64, t2281: f64, t35039: f64) -> (f64, f64, f64, f64) {
    let t45183 = t16503 * t14243 * t552 * t8430;
    let t45187 = t16503 * t16504 * t1598 * t8435;
    let t45189 = t34761 * t10072;
    let t45193 = t16503 * t35039 * t2281 * t1502;
    (t45183, t45187, t45189, t45193)
}
