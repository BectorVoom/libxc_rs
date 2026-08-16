//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 829/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk829(t6548: f64, t6633: f64, t393: f64, t1820: f64, t5036: f64, t3330: f64, t143: f64, t6432: f64, t3399: f64, t3400: f64, t6272: f64, t1154: f64, t1646: f64, t5153: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6634 = t6548 + t6633;
    let t6635 = t6634 * t393;
    let t6637 = 2.0_f64 * t5036 * t1820;
    let t6638 = t1820 * t1820;
    let t6640 = 2.0_f64 * t3330 * t6638;
    let t6641 = t6432 * t143;
    let t6661 = t3399 * t3400 * t6272;
    let t6665 = t1154 * t5153 * t1646;
    (t6634, t6635, t6637, t6638, t6640, t6641, t6661, t6665)
}
