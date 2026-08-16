//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1243/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1243(t5: f64, t56203: f64, t127: f64, t151: f64, t2024: f64, t2113: f64, t2124: f64, t2159: f64, t3467: f64, t48906: f64, t48922: f64, t48924: f64, t48960: f64, t48962: f64, t48990: f64, t48992: f64, t56110: f64, t56123: f64, t56178: f64, t56404: f64, t673: f64, t675: f64, t696: f64, t7129: f64) -> f64 {
    let t56540 = t5 * t56203;
    let t56553 = -0.48681704342817043984e1_f64 * t48906 - 0.31295381363239528276e1_f64 * t2124 * t7129 * t56123 + 0.69545291918310062836e0_f64 * t3467 * t151 * t56110 - 0.33855833396020740576e1_f64 * t48922 + 0.9736340868563408797e1_f64 * t48924 + 0.3173984380876944429e0_f64 * t2159 * t696 * t56178 - 0.48681704342817043985e1_f64 * t48960 - 0.48681704342817043985e1_f64 * t48962 - 0.48681704342817043984e1_f64 * t48990 - 0.33855833396020740576e1_f64 * t48992 + 0.52158968938732547127e0_f64 * t2113 * t675 * t56540 * t2024 - 0.26079484469366273564e0_f64 * t673 * t675 * t56540 * t127 - 0.86931614897887578546e-1_f64 * t673 * t675 * t56404 * t127;
    t56553
}
