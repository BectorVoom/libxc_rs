//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 346/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk346(t43: f64, t1193: f64, t588: f64, t592: f64, t596: f64, t600: f64, t604: f64, t608: f64, t612: f64, t1192: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t45 = 0.135e1_f64 < t43;
    let t1196 = t588 * t1193;
    let t1198 = t592 * t1193;
    let t1200 = t596 * t1193;
    let t1202 = t600 * t1193;
    let t1204 = t604 * t1193;
    let t1206 = t608 * t1193;
    let t1208 = t612 * t1193;
    let t1211 = piecewise3(t45, 0.0_f64, t1192);
    (t1196, t1198, t1200, t1202, t1204, t1206, t1208, t1211)
}
