//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2253/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2253(t3704: f64, t5293: f64, t1121: f64, t1214: f64, t606: f64, t1250: f64, t17353: f64, t1802: f64, t3147: f64, t3597: f64, t3594: f64, t1244: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17509 = 0.15244095330869239812e-2_f64 * t5293 * t3704;
    let t17512 = t1214 * t1121;
    let t17513 = t17512 * t606;
    let t17514 = t1250 * t17513;
    let t17515 = t17353 * t17514;
    let t17523 = t1802 * t3147;
    let t17524 = t3597 * t17523;
    let t17525 = t3594 * t17524;
    let t17528 = t1244 * t17523;
    (t17509, t17512, t17513, t17514, t17515, t17524, t17525, t17528)
}
