//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 611/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk611(t140: f64, t430: f64, t728: f64, t1922: f64, t4265: f64, t1925: f64, t299: f64, t41: f64, t4594: f64, t4597: f64, t702: f64, t3290: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5242 = 0.88437037037037037037e-2_f64 * t140 * t430 * t728;
    let t5243 = t4265 * t1922;
    let t5246 = t140 * t299 * t1925;
    let t5248 = t41 * t4594;
    let t5249 = t702 * t4597;
    let t5251 = t5248 * t5249 * t3290;
    (t5242, t5243, t5246, t5248, t5249, t5251)
}
