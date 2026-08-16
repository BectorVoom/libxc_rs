//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 460/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk460(t194: f64, t618: f64, t1412: f64, t171: f64, t433: f64, t1415: f64, t385: f64, t1413: f64, t381: f64, t1131: f64, t577: f64, t155: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5395 = t194 * t618;
    let t5400 = t1412 * t171;
    let t5402 = 0.11696447245269292414e1_f64 * t5400 * t433;
    let t5404 = t385 * t1415;
    let t5407 = 8.0_f64 * t381 * t1413;
    let t5409 = 8.0_f64 * t385 * t1413;
    let t5419 = t577 * t1131;
    let t5420 = t155 * t5419;
    (t5395, t5402, t5404, t5407, t5409, t5420)
}
