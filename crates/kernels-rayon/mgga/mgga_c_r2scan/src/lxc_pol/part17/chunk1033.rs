//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1033/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1033(t325: f64, t326: f64, t6691: f64, t1337: f64, t1347: f64, t260: f64, t277: f64, t481: f64, t1541: f64, t57: f64, t2146: f64, t2182: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19203 = t325 / t6691 / t326;
    let t19309 = 1.0_f64 / t1347 / t1337;
    let t19326 = t1347 * t1347;
    let t19327 = 1.0_f64 / t19326;
    let t19790 = t260 * t277;
    let t19791 = t19790 * t481;
    let t19839 = t57 * t1541;
    let t19865 = t2182 * t2146;
    (t19203, t19309, t19327, t19790, t19791, t19839, t19865)
}
