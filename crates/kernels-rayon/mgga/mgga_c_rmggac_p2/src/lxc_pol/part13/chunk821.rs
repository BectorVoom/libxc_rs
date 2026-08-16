//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 821/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk821(t16503: f64, t35039: f64, t38523: f64, t7448: f64, t34761: f64, t9171: f64, t34760: f64, t8450: f64, t7463: f64, t3369: f64, t34975: f64, t38444: f64, t495: f64, t8440: f64) -> (f64, f64, f64, f64, f64) {
    let t38526 = t16503 * t35039 * t38523 * t7448;
    let t38528 = t34761 * t9171;
    let t38530 = t8450 * t34760;
    let t38531 = t38530 * t7463;
    let t38539 = t34975 * t3369 * t8440 * t38444 * t495;
    (t38526, t38528, t38530, t38531, t38539)
}
