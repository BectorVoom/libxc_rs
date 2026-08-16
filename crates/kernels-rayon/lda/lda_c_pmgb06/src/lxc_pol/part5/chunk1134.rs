//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1134/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1134(t20613: f64, t1447: f64, t7674: f64, t2485: f64, t5187: f64, t2002: f64, t6250: f64, t1420: f64, t7574: f64, t20601: f64, t20602: f64, t20603: f64, t20604: f64, t20608: f64, t20610: f64, t20612: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20614 = 2.0_f64 / 45.0_f64 * t20613;
    let t20615 = t1447 * t7674;
    let t20616 = 2.0_f64 / 45.0_f64 * t20615;
    let t20618 = t5187 * t2485 / 9.0_f64;
    let t20620 = t2002 * t6250 / 9.0_f64;
    let t20622 = t1420 * t7574 / 15.0_f64;
    let t20623 = -t20601 - t20602 - t20603 + t20604 - t20608 + t20610 + t20612 + t20614 + t20616 + t20618 + t20620 + t20622;
    (t20614, t20616, t20618, t20620, t20622, t20623)
}
