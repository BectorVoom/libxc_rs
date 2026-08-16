//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 795/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk795(t1232: f64, t3260: f64, t4416: f64, t4415: f64, t1642: f64, t3267: f64, t3275: f64, t3273: f64, t3179: f64, t3191: f64, t189: f64, t4377: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4417 = t3260 * t1232;
    let t4418 = t4416 * t4417;
    let t4419 = t4415 * t4418;
    let t4422 = t3267 * t1642;
    let t4424 = t4416 * t3275;
    let t4425 = t3273 * t4424;
    let t4428 = 0.5848223622634646207e0_f64 * t3179;
    let t4429 = 0.18311447306006545054e-3_f64 * t3191;
    let t4430 = t4377 * t189;
    (t4417, t4419, t4422, t4425, t4428, t4429, t4430)
}
