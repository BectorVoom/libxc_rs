//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 258/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk258(t1222: f64, t365: f64, t45: f64, t370: f64, t1246: f64, t373: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1258 = 0.92708333333333333333e-2_f64 * t1222;
    let t1264 = t45 * t365;
    let t1265 = t370 * t370;
    let t1266 = 1.0_f64 / t1265;
    let t1268 = 0.301925e0_f64 * t1222;
    let t1271 = 0.16557e0_f64 * t1246;
    let t1275 = 1.0_f64 / t373;
    (t1258, t1264, t1265, t1266, t1268, t1271, t1275)
}
