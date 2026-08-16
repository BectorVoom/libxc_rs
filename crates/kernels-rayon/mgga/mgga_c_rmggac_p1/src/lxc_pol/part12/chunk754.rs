//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 754/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk754(t2186: f64, t7687: f64, t638: f64, t7292: f64, t7301: f64, t2046: f64, t7297: f64, t7389: f64, t7305: f64, t7393: f64, t132: f64, t26007: f64, t271: f64, t298: f64, t34: f64, t4766: f64, t637: f64, t71: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35473 = t2186 * t7687;
    let t35478 = t638 * t7292 * t7301;
    let t35481 = t2046 * t7297 * t7389;
    let t35484 = t638 * t7292 * t7305;
    let t35487 = t2046 * t7297 * t7393;
    let t35496 = t26007 / t34 / t298 * t271 * t71 * t132 * t4766 * t637;
    (t35473, t35478, t35481, t35484, t35487, t35496)
}
