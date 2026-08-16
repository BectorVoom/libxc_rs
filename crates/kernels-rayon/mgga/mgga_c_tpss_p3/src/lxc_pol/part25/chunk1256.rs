//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1256/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1256(t1812: f64, t4706: f64, t1364: f64, t1398: f64, t1692: f64, t18812: f64, t198: f64, t20514: f64, t207: f64, t21262: f64, t21658: f64, t2439: f64, t3552: f64, t4701: f64, t4802: f64, t4806: f64, t5853: f64, t6354: f64, t823: f64) -> (f64, f64) {
    let t21678 = t1812 * t4706;
    let t21701 = t198 * t207 * t21658 * t823 + 6.0_f64 * t1364 * t2439 * t6354 - 2.0_f64 * t1398 * t1692 * t20514 + 2.0_f64 * t1692 * t18812 * t4806 - t1692 * t4802 * t5853 + 3.0_f64 * t1812 * t2439 * t4701 - 6.0_f64 * t21262 * t2439 * t5853 + 6.0_f64 * t21678 * t3552;
    (t21678, t21701)
}
