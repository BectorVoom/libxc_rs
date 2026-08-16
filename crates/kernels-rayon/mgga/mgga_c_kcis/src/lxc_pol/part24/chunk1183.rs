//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1183/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1183(t26929: f64, t9588: f64, t14874: f64, t283: f64, t3463: f64, t376: f64, t1169: f64, t14654: f64, t990: f64) -> (f64, f64, f64, f64, f64) {
    let t95391 = t9588 * t26929;
    let t95416 = t14874 * t283;
    let t95463 = t3463 * t376;
    let t95474 = t1169 * t376;
    let t95524 = t14654 * t283 * t990;
    (t95391, t95416, t95463, t95474, t95524)
}
