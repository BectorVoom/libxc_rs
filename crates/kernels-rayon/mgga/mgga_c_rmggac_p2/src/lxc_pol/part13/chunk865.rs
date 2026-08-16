//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 865/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk865(t3350: f64, t39207: f64, t7751: f64, t674: f64, t7715: f64, t8687: f64, t1997: f64, t7243: f64, t8576: f64, t1973: f64, t16156: f64, t9138: f64) -> (f64, f64, f64, f64, f64) {
    let t39277 = t39207 * t3350;
    let t39278 = t39277 * t7751;
    let t39281 = t8687 * t7715 * t674;
    let t39282 = t39281 * t1997;
    let t39284 = t8576 * t7243;
    let t39285 = t39284 * t1973;
    let t39289 = t16156 * t9138;
    (t39277, t39278, t39282, t39285, t39289)
}
