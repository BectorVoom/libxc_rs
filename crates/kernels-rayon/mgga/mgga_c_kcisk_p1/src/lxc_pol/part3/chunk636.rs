//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 636/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk636(t458: f64, t476: f64, t139: f64, t201: f64, t41: f64, t3529: f64, t451: f64, t1337: f64, t469: f64, t485: f64, t1284: f64, t4229: f64, t491: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6267 = t476 * t458;
    let t6278 = t139 * t201 * t41;
    let t6279 = t3529 * t451;
    let t6287 = t1337 * t451;
    let t6316 = t485 * t469;
    let t6317 = t41 * t1284;
    let t6321 = t491 * t4229;
    (t6267, t6278, t6279, t6287, t6316, t6317, t6321)
}
