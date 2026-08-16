//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1343/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1343(t154: f64, t3584: f64, t3241: f64, t636: f64, t52: f64, t1094: f64, t3312: f64, t3311: f64, t419: f64, t409: f64, t11135: f64, t10292: f64, t281: f64, t415: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11145 = t154 * t3584;
    let t11147 = 1.0_f64 / t3241 / t636;
    let t11152 = t3241 * t52;
    let t11153 = 1.0_f64 / t11152;
    let t11185 = t1094 * t3312;
    let t11189 = 1.0_f64 / t3311 / t419;
    let t11190 = t409 * t11189;
    let t11195 = 0.93011851851851851854e0_f64 * t11135;
    let t11203 = t281 * t10292 * t415;
    (t11145, t11147, t11153, t11185, t11190, t11195, t11203)
}
