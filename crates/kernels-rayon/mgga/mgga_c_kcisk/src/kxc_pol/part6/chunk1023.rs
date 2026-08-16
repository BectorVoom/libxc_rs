//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1023/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1023(t1398: f64, t30298: f64, t1383: f64, t1375: f64, t30290: f64, t30238: f64, t1471: f64, t30233: f64, t3661: f64, t457: f64, t1186: f64, t158: f64, t165: f64, t173: f64, t25495: f64) -> f64 {
    let t30803 = t1398 * t30298;
    let t30806 = t1383 * t30298;
    let t30809 = t1375 * t30290;
    let t30812 = t1375 * t30238;
    let t30815 = t1471 * t30233;
    let t30818 = t1383 * t30238;
    let t30821 = t3661 * t30233;
    let t30824 = t1398 * t30238;
    let t30827 = t457 * t30233;
    let t30830 = t1186 * t30290;
    let t30833 = t1375 * t30298;
    let t30836 = 0.79249999999999999999e-2_f64 * t25495 + 0.30247875e-4_f64 * t173 * t30803 + 0.4755e-2_f64 * t165 * t30806 + 0.403305e-4_f64 * t173 * t30809 - 0.3513e-2_f64 * t158 * t30812 + 0.78066666666666666667e-3_f64 * t158 * t30815 + 0.7925e-3_f64 * t165 * t30818 - 0.17611111111111111111e-3_f64 * t165 * t30821 + 0.50413125e-5_f64 * t173 * t30824 + 0.22405833333333333333e-5_f64 * t173 * t30827 + 0.317e-2_f64 * t165 * t30830 - 0.21078e-1_f64 * t158 * t30833;
    t30836
}
