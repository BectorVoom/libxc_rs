//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 255/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk255(t357: f64, t346: f64, t347: f64, t1222: f64, t344: f64, t164: f64, t313: f64, t353: f64, t352: f64, t81: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1232 = t357 * t357;
    let t1233 = 1.0_f64 / t1232;
    let t1234 = t346 * t1233;
    let t1235 = 1.0_f64 / t347;
    let t1240 = 0.29896666666666666667e0_f64 * t1222;
    let t1242 = f64::sqrt(t344);
    let t1246 = t353 * t164 * t313;
    let t1247 = 0.16431333333333333333e0_f64 * t1246;
    let t1248 = t352 * t81;
    (t1232, t1233, t1234, t1235, t1240, t1242, t1246, t1247, t1248)
}
