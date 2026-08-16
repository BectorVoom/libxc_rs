//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 957/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk957(t1407: f64, t3805: f64, t1333: f64, t3916: f64, t13854: f64, t470: f64, t468: f64, t415: f64, t3494: f64, t3742: f64, t1415: f64, t1411: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t14160 = t3805 * t1407;
    let t14162 = t1333 * t3916;
    let t14164 = sigma0 * t13854;
    let t14165 = t14164 * t470;
    let t14166 = t468 * t14165;
    let t14167 = t415 * t14166;
    let t14169 = t3494 * t3742;
    let t14170 = t1415 * t14169;
    let t14171 = t1411 * t14170;
    (t14160, t14162, t14164, t14167, t14171)
}
