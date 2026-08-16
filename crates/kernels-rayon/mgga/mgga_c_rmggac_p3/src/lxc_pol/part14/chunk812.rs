//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 812/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk812(t574: f64, t638: f64, t639: f64, t7215: f64, t1656: f64, t2164: f64, t5280: f64, t640: f64, t5542: f64, t8601: f64, t674: f64, t2004: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38340 = t638 * t639 * t7215 * t574;
    let t38344 = t638 * t639 * t2164 * t1656;
    let t38348 = t638 * t639 * t640 * t5280;
    let t38350 = t8601 * t5542;
    let t38351 = t38350 * t674;
    let t38352 = t38351 * t2004;
    (t38340, t38344, t38348, t38350, t38351, t38352)
}
