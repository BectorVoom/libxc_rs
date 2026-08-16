//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 942/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk942(t1727: f64, t3303: f64, t3275: f64, t10415: f64, t1670: f64, t127: f64, t2840: f64, t368: f64, t1109: f64, t2844: f64, t14303: f64, t1114: f64) -> (f64, f64, f64, f64, f64) {
    let t14312 = t3303 * t1727;
    let t14313 = t14312 * t3275;
    let t14316 = t10415 * t1670;
    let t14317 = t14316 * t3275;
    let t14321 = t127 * t368 * t2840;
    let t14322 = t1109 * t2844;
    let t14323 = t14322 * t14303;
    let t14326 = t1114 * t2844;
    (t14313, t14317, t14321, t14323, t14326)
}
