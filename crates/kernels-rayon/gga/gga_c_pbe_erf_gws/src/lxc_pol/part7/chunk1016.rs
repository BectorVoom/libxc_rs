//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1016/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1016(t1290: f64, t1294: f64, t174: f64, t331: f64, t1305: f64, t1314: f64, t1319: f64, t470: f64, t1434: f64, t4813: f64, t18424: f64, t18428: f64, t18432: f64, t18435: f64, t18439: f64, t18441: f64, t18445: f64, t18448: f64, t18452: f64) -> (f64, f64, f64, f64) {
    let t18456 = 0.2291123905095794067e1_f64 * t174 * t331 * t1290 * t1294;
    let t18460 = 0.21053604230838734656e2_f64 * t470 * t1319 * t1314 * t1305;
    let t18461 = t1434 * t4813;
    let t18462 = 0.22787712934626154593e-2_f64 * t18461;
    let t18463 = t18424 - t18428 + t18432 - t18435 + t18439 - t18441 - t18445 - t18448 - t18452 + t18456 - t18460 - t18462;
    (t18456, t18460, t18462, t18463)
}
