//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1123/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1123(t1353: f64, t5591: f64, t4012: f64, t828: f64, t1868: f64, t3889: f64, t221: f64, t5627: f64, t9921: f64, t3978: f64, t13583: f64, t13585: f64, t13593: f64, t13599: f64, t13612: f64, t13615: f64, t9278: f64, t9308: f64, t9316: f64, t9320: f64, t9325: f64, t9329: f64, t9333: f64, t9374: f64, t9389: f64, t9391: f64) -> (f64, f64, f64, f64) {
    let t13867 = t5591 * t1353;
    let t13869 = t4012 * t828 * t13867;
    let t13872 = t1868 * t3889;
    let t13874 = t4012 * t828 * t13872;
    let t13877 = t221 * t5627;
    let t13878 = t9921 * t13877;
    let t13880 = 0.50820002809285328225e-3_f64 * t3978 * t13878;
    let t13881 = t13583 + t13585 - t9278 + t9308 + t9316 + t9320 - t9325 + t9329 + t9333 - t9374 - t13593 - t9389 - t13599 - t9391 - t13612 - t13615;
    (t13869, t13874, t13880, t13881)
}
