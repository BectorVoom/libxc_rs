//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 622/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk622(t289: f64, t8188: f64, t2232: f64, t275: f64, t2231: f64, t302: f64, t72: f64, t1347: f64, t703: f64, t1288: f64, t702: f64, t7897: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8189 = t289 * t8188;
    let t8190 = 0.4726e1_f64 * t8189;
    let t8191 = t275 * t2232;
    let t8198 = t302 * t2231;
    let t8199 = t72 * t8198;
    let t8200 = 2.0_f64 * t8199;
    let t8201 = t1347 * t703;
    let t8202 = t1288 * t702;
    let t8203 = t72 * t8202;
    let t8204 = 0.2993560425465952141e-1_f64 * t7897;
    (t8190, t8191, t8198, t8200, t8201, t8202, t8203, t8204)
}
