//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1057/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1057(t1987: f64, t47854: f64, t1990: f64, t1979: f64, t1982: f64, t458: f64, t9774: f64, t38530: f64, t8422: f64, t8427: f64, t46522: f64, t8630: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47988 = t47854 * t1987;
    let t47990 = t47854 * t1990;
    let t47994 = t9774 * t458 * t1979 * t1982;
    let t47996 = t38530 * t8422;
    let t48000 = t38530 * t8427;
    let t48009 = t8630 * t46522;
    (t47988, t47990, t47994, t47996, t48000, t48009)
}
