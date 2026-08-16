//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 424/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk424(t4052: f64, t4160: f64, t1105: f64, t362: f64, t135: f64, t1091: f64, t376: f64, t1108: f64, t150: f64, t147: f64, t1109: f64, t1062: f64, t401: f64) -> (f64, f64, f64, f64, f64) {
    let t4203 = t4052 * t4160;
    let t4207 = 1.0_f64 / t1105 / t362;
    let t4208 = t135 * t4207;
    let t4209 = t1091 * t376;
    let t4211 = 1.0_f64 / t1108 / t150;
    let t4212 = t4209 * t4211;
    let t4214 = 0.51726012919273400301e3_f64 * t4208 * t4212;
    let t4216 = 1.0_f64 / t1105 / t147;
    let t4217 = t135 * t4216;
    let t4218 = t4209 * t1109;
    let t4220 = 0.96491876992155210402e2_f64 * t4217 * t4218;
    let t4221 = t1062 * t401;
    (t4203, t4209, t4214, t4220, t4221)
}
