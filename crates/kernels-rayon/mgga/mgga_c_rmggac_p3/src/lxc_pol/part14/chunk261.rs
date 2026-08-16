//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 261/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk261(t198: f64, t673: f64, t1193: f64, t209: f64, t476: f64, t446: f64, t221: f64, t1149: f64, t205: f64, t1156: f64, t23: f64, t1144: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1194 = t673 * t198;
    let t1195 = t1193 * t1194;
    let t1196 = t476 * t209;
    let t1197 = t1196 * t446;
    let t1198 = t221 * t1197;
    let t1201 = t1149 * t205;
    let t1205 = t23 * t1156;
    let t1206 = t1205 * t1144;
    (t1194, t1195, t1196, t1198, t1201, t1206)
}
