//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1247/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1247(t16836: f64, t3717: f64, t27357: f64, t5440: f64, t28347: f64, t94246: f64, t27369: f64, t1464: f64, t28360: f64, t94216: f64, t27364: f64, t28382: f64) -> (f64, f64, f64, f64, f64) {
    let t98359 = t16836 * t3717;
    let t98361 = t98359 * t5440 * t27357;
    let t98364 = t94246 * t28347;
    let t98365 = t27369 * t98364;
    let t98370 = t1464 * t94216 * t28360;
    let t98373 = t1464 * t27364 * t28382;
    (t98361, t98364, t98365, t98370, t98373)
}
