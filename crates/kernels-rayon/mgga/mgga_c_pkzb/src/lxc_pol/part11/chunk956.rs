//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 956/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk956(t10481: f64, t195: f64, t1062: f64, t3359: f64, t3507: f64, t998: f64, t8718: f64, t6804: f64, t6811: f64, t6819: f64, t6821: f64, t6823: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10482 = t10481 * t195;
    let t10483 = t3359 * t1062;
    let t10484 = 3.0_f64 * t10483;
    let t10485 = t998 * t3507;
    let t10486 = 3.0_f64 * t10485;
    let t10487 = 0.54934341918019635162e-3_f64 * t8718;
    let t10488 = 0.73245789224026180216e-3_f64 * t6804;
    let t10489 = 24.0_f64 * t6811;
    let t10490 = 60.0_f64 * t6819;
    let t10491 = 36.0_f64 * t6821;
    let t10492 = 96.0_f64 * t6823;
    (t10482, t10484, t10486, t10487, t10488, t10489, t10490, t10491, t10492)
}
