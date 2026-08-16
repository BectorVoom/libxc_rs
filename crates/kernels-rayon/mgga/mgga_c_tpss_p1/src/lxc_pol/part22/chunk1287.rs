//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1287/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1287(t1233: f64, t4459: f64, t1232: f64, t13763: f64, t1268: f64, t4519: f64, t1625: f64, t3202: f64, t1206: f64, t1364: f64, t2436: f64, t10514: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43908 = t1233 * t4459;
    let t43933 = t13763 * t1232;
    let t43998 = t4519 * t1268;
    let t44045 = t1625 * t3202;
    let t44070 = t1206 * t4519;
    let t44169 = t2436 * t1364;
    let t44170 = t44169 * t10514;
    (t43908, t43933, t43998, t44045, t44070, t44170)
}
