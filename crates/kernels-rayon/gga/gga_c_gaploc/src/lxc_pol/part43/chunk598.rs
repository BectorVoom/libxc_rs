//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 598/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk598(t10513: f64, t10526: f64, t10525: f64, t19: f64, t4524: f64, t60: f64, t584: f64) -> (f64, f64, f64, f64) {
    let t10527 = t10526 * t10513;
    let t10529 = 0.21450293971110256001e1_f64 * t10525 * t10527;
    let t10530 = t4524 * t19;
    let t10531 = t10530 * t60;
    let t10532 = t584 * t10531;
    (t10529, t10530, t10531, t10532)
}
