//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1026/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1026(t1383: f64, t30273: f64, t1186: f64, t30294: f64, t1398: f64, t14083: f64, t14084: f64, t165: f64, t173: f64, t20781: f64, t20783: f64, t25538: f64, t25540: f64, t25542: f64, t25544: f64, t30852: f64, t3891: f64) -> f64 {
    let t30858 = t1383 * t30273;
    let t30861 = t1186 * t30294;
    let t30864 = t1398 * t30273;
    let t30873 = 0.46615850170166761884e-3_f64 * t3891 * t30852 - t14083 + t14084 - 0.4755e-2_f64 * t165 * t30858 - 0.1585e-2_f64 * t165 * t30861 - 0.30247875e-4_f64 * t173 * t30864 - 0.35867157975189532869e-1_f64 * t25538 + 0.31077233446777841256e-3_f64 * t25540 + 0.71734315950379065738e-1_f64 * t25542 - 0.93231700340333523768e-3_f64 * t25544 + 0.71734315950379065738e-1_f64 * t20781 - 0.93231700340333523768e-3_f64 * t20783;
    t30873
}
