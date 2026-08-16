//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 818/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk818(t4534: f64, t4582: f64, t1123: f64, t198: f64, t1129: f64, t1539: f64, t1160: f64, t1297: f64, t1535: f64, t502: f64, t535: f64, t17: f64, t2849: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4583 = t4534 + t4582;
    let t4711 = t198 * t1123;
    let t4714 = t198 * t1129;
    let t4851 = t198 * t1539;
    let t4861 = t1160 * t1297;
    let t5198 = t502 * t1535;
    let t5204 = t535 * t1535;
    let t5471 = t17 * t2849;
    (t4583, t4711, t4714, t4851, t4861, t5198, t5204, t5471)
}
