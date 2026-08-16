//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1355/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1355(t22335: f64, t4292: f64, t17470: f64, t21910: f64, t5903: f64, t1497: f64, t6917: f64, t5909: f64, t4260: f64, t2035: f64, t6041: f64, t22252: f64, t6011: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22336 = t4292 * t22335;
    let t22338 = t17470 * t21910;
    let t22339 = t5903 * t22338;
    let t22341 = t6917 * t1497;
    let t22342 = t5909 * t22341;
    let t22343 = t4260 * t22342;
    let t22345 = t2035 * t6041;
    let t22348 = t6011 * t22252;
    (t22336, t22339, t22341, t22343, t22345, t22348)
}
