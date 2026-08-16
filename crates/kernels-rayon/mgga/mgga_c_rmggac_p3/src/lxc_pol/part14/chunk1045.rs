//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1045/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1045(t40965: f64, t8620: f64, t22: f64, t235: f64, t34812: f64, t40978: f64, t16503: f64, t35039: f64, t571: f64, t7461: f64, t34764: f64, t8457: f64) -> (f64, f64, f64, f64) {
    let t41735 = t8620 * t40965;
    let t41736 = 0.36366215538993788972e-1_f64 * t41735;
    let t41738 = t235 * t34812 * t22;
    let t41739 = t41738 * t40978;
    let t41745 = t16503 * t35039 * t571 * t7461;
    let t41747 = t34764 * t8457;
    (t41736, t41739, t41745, t41747)
}
