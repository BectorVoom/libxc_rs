//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 450/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk450(t382: f64, t442: f64, t143: f64, t1055: f64, t142: f64, t179: f64, t139: f64) -> (f64, f64, f64, f64, f64) {
    let t3485 = t382 * t442;
    let t3499 = 2.0_f64 * t143;
    let t3500 = 2.0_f64 * t1055;
    let t3516 = t179 * t142;
    let t3517 = t139 * t3516;
    (t3485, t3499, t3500, t3516, t3517)
}
