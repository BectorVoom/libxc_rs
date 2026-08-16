//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 640/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk640(t5420: f64, t68: f64, t1287: f64, t5039: f64, t5161: f64, t5045: f64, t5190: f64, t5208: f64, t5212: f64, t5068: f64, t1490: f64, t5081: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5421 = t5420 * t68;
    let t5422 = t5421 * t1287;
    let t5426 = 1.2466946262544771_f64 * t5039;
    let t5430 = 8.333333333333334_f64 * t5161;
    let t5439 = 0.8311297508363181_f64 * t5045;
    let t5440 = 0.6944444444444444_f64 * t5190;
    let t5445 = 6.25_f64 * t5208;
    let t5446 = 6.25_f64 * t5212;
    let t5448 = 0.2770432502787727_f64 * t5068;
    let t5455 = t1490 * t5081;
    (t5422, t5426, t5430, t5439, t5440, t5445, t5446, t5448, t5455)
}
