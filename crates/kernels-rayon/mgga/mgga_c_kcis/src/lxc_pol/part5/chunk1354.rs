//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1354/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1354(t20956: f64, t4261: f64, t4260: f64, t17391: f64, t5913: f64, t21799: f64, t6028: f64, t17508: f64, t17504: f64, t4122: f64, t6012: f64, t20934: f64, t4293: f64) -> (f64, f64, f64, f64, f64) {
    let t22324 = t4261 * t20956;
    let t22325 = t4260 * t22324;
    let t22327 = t17391 * t5913;
    let t22329 = t6028 * t21799;
    let t22330 = t17508 * t22329;
    let t22332 = t4122 * t17504;
    let t22333 = t22332 * t6012;
    let t22335 = t4293 * t20934;
    (t22325, t22327, t22330, t22333, t22335)
}
