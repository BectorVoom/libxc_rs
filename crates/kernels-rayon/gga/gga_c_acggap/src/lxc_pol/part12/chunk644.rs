//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 644/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk644(t157: f64, t5079: f64, t175: f64, t398: f64, t1413: f64, t935: f64, t506: f64, t879: f64, t368: f64, t384: f64, t3476: f64, t527: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5080 = t5079 * t157;
    let t5082 = t398 * t175 * t5080;
    let t5086 = 0.42874018118069736972e-3_f64 * t935 * t1413;
    let t5087 = t506 * t879;
    let t5089 = t398 * t368 * t5087;
    let t5090 = t384 * t5089;
    let t5092 = t3476 * t527;
    (t5080, t5082, t5086, t5087, t5089, t5090, t5092)
}
