//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1036/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1036(t40031: f64, t8571: f64, t40092: f64, t41576: f64, t236: f64, t618: f64, t1981: f64, t3134: f64, t8512: f64, t10100: f64, t3352: f64, t495: f64, t8517: f64) -> (f64, f64, f64, f64, f64) {
    let t47598 = t8571 * t40031;
    let t47600 = t8571 * t40092;
    let t47602 = t8571 * t41576;
    let t47604 = t236 * t618;
    let t47607 = t8512 * t1981 * t3134 * t47604;
    let t47612 = t8517 * t3352 * t236 * t10100 * t495;
    (t47598, t47600, t47602, t47607, t47612)
}
