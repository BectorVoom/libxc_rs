//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 822/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk822(t34761: f64, t8422: f64, t16503: f64, t35039: f64, t7461: f64, t8440: f64, t16504: f64, t34975: f64, t38416: f64, t495: f64, t7491: f64, t8355: f64) -> (f64, f64, f64, f64) {
    let t38541 = t34761 * t8422;
    let t38545 = t16503 * t35039 * t8440 * t7461;
    let t38550 = t34975 * t16504 * t8440 * t38416 * t495;
    let t38552 = t7491 * t8355;
    (t38541, t38545, t38550, t38552)
}
