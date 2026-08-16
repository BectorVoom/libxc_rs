//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 330/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk330(t1236: f64, t407: f64, t1160: f64, t159: f64, t441: f64, t322: f64, t381: f64, t452: f64, t879: f64, t180: f64, t939: f64, t945: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1237 = t1236 * t407;
    let t1238 = t1160 * t1237;
    let t1240 = t159 * t441;
    let t1241 = t1240 * t322;
    let t1242 = t381 * t1241;
    let t1244 = t452 * t879;
    let t1246 = 0.65854491829355115987e0_f64 * t381 * t1244;
    let t1247 = t939 * t180;
    let t1248 = t1247 * t945;
    (t1237, t1238, t1240, t1241, t1242, t1244, t1246, t1248)
}
