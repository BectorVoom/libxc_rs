//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1245/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1245(t14400: f64, t2811: f64, t1003: f64, t1008: f64, t1000: f64, t1245: f64, t26687: f64, t7703: f64, t1009: f64, t9494: f64, t26685: f64, t44575: f64, t7705: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t93426 = t14400 * t2811;
    let t93427 = t1003 * t1008;
    let t93435 = t1245 * t1000;
    let t93436 = t93435 * t26687;
    let t93437 = t7703 * t93436;
    let t93463 = t1009 * t9494;
    let t93468 = t26685 * t93436;
    let t93471 = t7703 * t44575 * t7705;
    (t93426, t93427, t93435, t93437, t93463, t93468, t93471)
}
