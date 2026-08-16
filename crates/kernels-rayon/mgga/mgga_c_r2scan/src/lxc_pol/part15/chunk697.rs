//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 697/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk697(t5317: f64, t720: f64, t748: f64, t234: f64, t1654: f64, t1861: f64, t1860: f64, t170: f64, t1871: f64, t597: f64, t1853: f64, t625: f64, t645: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5318 = t720 * t5317;
    let t5319 = t748 * t5318;
    let t5321 = 0.17315859105681463759e2_f64 * t234 * t5319;
    let t5322 = t1654 * t1861;
    let t5323 = t1860 * t5322;
    let t5325 = t170 * t1871;
    let t5326 = t597 * t5325;
    let t5327 = t1860 * t5326;
    let t5331 = 0.71233333333333333332e-1_f64 * t625 * t1853 * t645;
    (t5321, t5322, t5323, t5325, t5326, t5327, t5331)
}
