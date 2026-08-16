//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 766/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk766(t1025: f64, t4079: f64, t1509: f64, t2885: f64, t1027: f64, t1032: f64, t1515: f64, t673: f64, t2895: f64, t4047: f64, t141: f64, t1038: f64, t4052: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4080 = t1025 * t4079;
    let t4087 = t2885 * t1509;
    let t4088 = t4087 * t1027;
    let t4090 = t1032 * t4079;
    let t4093 = t673 * t1515;
    let t4095 = t2895 * t4047;
    let t4096 = t141 * t4095;
    let t4098 = t1038 * t4052;
    (t4080, t4087, t4088, t4090, t4093, t4095, t4096, t4098)
}
