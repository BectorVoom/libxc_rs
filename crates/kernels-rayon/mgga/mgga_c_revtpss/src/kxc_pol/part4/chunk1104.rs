//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1104/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1104(t1450: f64, t5778: f64, t3889: f64, t5537: f64, t1353: f64, t13583: f64, t13585: f64, t13586: f64, t13593: f64, t13599: f64, t1868: f64, t3829: f64, t4139: f64, t5532: f64, t5536: f64, t9278: f64, t9308: f64, t9316: f64, t9320: f64, t9325: f64, t9329: f64, t9333: f64, t9374: f64, t9389: f64, t9391: f64, t9547: f64, t9599: f64) -> f64 {
    let t13600 = t5778 * t1450;
    let t13607 = t5537 * t3889;
    let t13610 = 12.0_f64 * t1353 * t13586 * t5536 + 6.0_f64 * t1353 * t13600 * t4139 + 3.0_f64 * t1868 * t4139 * t9547 - 3.0_f64 * t1868 * t4139 * t9599 + 6.0_f64 * t3829 * t5532 * t5536 + 6.0_f64 * t13607 * t5536 + t13583 + t13585 - t13593 - t13599 - t9278 + t9308 + t9316 + t9320 - t9325 + t9329 + t9333 - t9374 - t9389 - t9391;
    t13610
}
