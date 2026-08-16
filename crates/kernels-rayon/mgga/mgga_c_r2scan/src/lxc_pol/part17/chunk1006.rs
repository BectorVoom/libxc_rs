//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1006/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1006(t1060: f64, t3216: f64, t1058: f64, t2201: f64, t2207: f64, t3606: f64, t3613: f64, t3190: f64, t5103: f64, t2892: f64, t5095: f64, t3016: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12448 = t1060 * t3216;
    let t12450 = t2201 * t1058 * t12448;
    let t12453 = t2207 * t3613 * t3606;
    let t12455 = t1060 * t3190;
    let t12457 = t5103 * t1058 * t12455;
    let t12459 = t1060 * t2892;
    let t12461 = t5095 * t1058 * t12459;
    let t12463 = t1060 * t3016;
    (t12448, t12450, t12453, t12455, t12457, t12459, t12461, t12463)
}
