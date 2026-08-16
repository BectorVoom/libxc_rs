//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 503/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk503(t12: f64, t24: f64, t124: f64, t2557: f64, t207: f64, t8: f64, t1064: f64, t1429: f64, t439: f64, t333: f64, t1165: f64, t507: f64, zeta_threshold: f64) -> (f64, f64) {
    let t84 = t12 <= zeta_threshold;
    let t90 = t24 <= zeta_threshold;
    let t2559 = 0.19751673498613801407e-1_f64 * t2557 * t124;
    let t2562 = t207 * t8;
    let t2566 = piecewise3(t84, 0.0_f64, -2.0_f64 / 9.0_f64 * t1064 * t439 + 4.0_f64 / 3.0_f64 * t2562 * t1429);
    let t2569 = t333 * t8;
    let t2573 = piecewise3(t90, 0.0_f64, -2.0_f64 / 9.0_f64 * t1165 * t507 - 4.0_f64 / 3.0_f64 * t2569 * t1429);
    let t2575 = t2566 / 2.0_f64 + t2573 / 2.0_f64;
    (t2559, t2575)
}
