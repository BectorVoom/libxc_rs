//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1119/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1119(t1850: f64, t9633: f64, t9760: f64, t7173: f64, t9647: f64, t9648: f64, t29439: f64, t9652: f64, t2554: f64, t7064: f64, t7280: f64, t21665: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29455 = 0.17090058289204942853e-2_f64 * t1850 * t9633;
    let t29457 = 0.17090058289204942853e-2_f64 * t1850 * t9760;
    let t29471 = 0.1922631557535556071e-2_f64 * t9647 * t9648 * t7173;
    let t29473 = 0.2563508743380741428e-2_f64 * t29439 * t9652;
    let t29476 = 0.1281754371690370714e-2_f64 * t7064 * t7280 * t2554;
    let t29478 = 0.1281754371690370714e-2_f64 * t21665 * t9633;
    (t29455, t29457, t29471, t29473, t29476, t29478)
}
