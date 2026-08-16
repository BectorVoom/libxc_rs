//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1635/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1635(t15357: f64, t457: f64, t460: f64, t974: f64, t1716: f64, t698: f64, t1174: f64, t3435: f64, t4889: f64, t135: f64, t4930: f64, t1420: f64, t1887: f64, t337: f64) -> (f64, f64, f64, f64, f64) {
    let t15359 = t457 * t15357 * t460;
    let t15360 = t974 * t15359;
    let t15363 = t698 * t1716;
    let t15364 = t1174 * t15363;
    let t15366 = t4889 * t3435;
    let t15372 = t135 * t4930;
    let t15374 = 0.55555555555555555554e-3_f64 * t1174 * t15372;
    let t15376 = t1420 * t337 * t1887;
    (t15360, t15364, t15366, t15374, t15376)
}
