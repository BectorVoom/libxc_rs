//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2364/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2364(t100871: f64, t100873: f64, t100875: f64, t100879: f64, t100883: f64, t100885: f64, t100887: f64, t100890: f64, t100893: f64, t100897: f64, t100899: f64, t100902: f64, t105102: f64, t105105: f64, t105108: f64, t19534: f64, t24969: f64, t5456: f64, t5493: f64, t577: f64, t671: f64, t7423: f64) -> f64 {
    let t105115 = 0.45e1_f64 * t105102 * t577 + 0.135e2_f64 * t105105 * t671 + t100871 + t100873 + t100875 + 27.0_f64 * t105108 * t5456 + 0.135e2_f64 * t7423 * t19534 + t100879 + 0.135e2_f64 * t24969 * t5493 + t100883 + t100885 + t100887 + t100890 + t100893 + t100897 + t100899 + t100902;
    t105115
}
