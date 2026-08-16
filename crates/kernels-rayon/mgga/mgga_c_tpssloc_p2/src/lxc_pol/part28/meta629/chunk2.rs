//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1972/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1972(t86911: f64, t86916: f64, t86928: f64, t86940: f64, t86942: f64, t13029: f64, t2047: f64, t259: f64, t26700: f64, t2743: f64, t4142: f64, t7084: f64, t7842: f64, t82079: f64, t82082: f64, t82087: f64, t86933: f64, t9590: f64) -> (f64, f64, f64) {
    let t92402 = 0.52089578783527170489e-1_f64 * t86911;
    let t92406 = 0.3289868133696452873e-1_f64 * t86916;
    let t92415 = 0.16449340668482264365e-1_f64 * t86928;
    let t92425 = 0.16449340668482264365e-1_f64 * t86940;
    let t92426 = 0.76763589786250567036e-1_f64 * t86942;
    let t92428 = 0.82246703342411321825e-2_f64 * t82079 - t92415 + 0.3289868133696452873e-1_f64 * t82082 - 0.3289868133696452873e-1_f64 * t82087 - t9590 * t7842 + 2.0_f64 * t4142 * t7084 * t259 + 0.6579736267392905746e-1_f64 * t86933 + t13029 * t2047 * t259 + t92425 + t92426 - t26700 * t2743;
    (t92402, t92406, t92428)
}
