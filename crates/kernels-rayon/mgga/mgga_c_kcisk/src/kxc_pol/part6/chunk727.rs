//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 727/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk727(t12825: f64, t458: f64, t12829: f64, t459: f64, t12951: f64, t13009: f64, t420: f64, t12974: f64, t1390: f64, t382: f64, t1412: f64, t453: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13220 = t12825 * t458;
    let t13221 = t459 * t12829;
    let t13233 = t459 * t12951;
    let t13244 = t13009 * t420;
    let t13263 = 0.12841111111111111111e-1_f64 * t12974;
    let t13293 = t382 * t1390;
    let t13327 = t1412 * t1412;
    let t13328 = 1.0_f64 / t13327;
    let t13329 = t453 * t13328;
    (t13220, t13221, t13233, t13244, t13263, t13293, t13328, t13329)
}
