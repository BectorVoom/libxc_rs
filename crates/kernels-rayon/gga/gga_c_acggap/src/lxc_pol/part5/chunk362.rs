//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 362/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk362(t1240: f64, t322: f64, t381: f64, t452: f64, t879: f64, t180: f64, t939: f64, t945: f64, t394: f64, t441: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1241 = t1240 * t322;
    let t1242 = t381 * t1241;
    let t1244 = t452 * t879;
    let t1246 = 0.65854491829355115987e0_f64 * t381 * t1244;
    let t1247 = t939 * t180;
    let t1248 = t1247 * t945;
    let t1251 = t394 * t441;
    (t1241, t1242, t1244, t1246, t1247, t1248, t1251)
}
