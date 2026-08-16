//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 569/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk569(t3106: f64, t3165: f64, t349: f64, t1050: f64, t225: f64, t1053: f64, t386: f64, t68: f64, t1065: f64, t1057: f64, t3112: f64, t3032: f64, t3127: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3166 = t3106 + t3165;
    let t3167 = t349 * t3166;
    let t3169 = t1050 * t225;
    let t3173 = 1.0_f64 / t1053 / t386;
    let t3174 = t68 * t3173;
    let t3175 = t1065 * t1065;
    let t3176 = t3174 * t3175;
    let t3180 = t3112 * t1057;
    let t3185 = t3032 * t3127;
    (t3166, t3167, t3169, t3174, t3175, t3176, t3180, t3185)
}
