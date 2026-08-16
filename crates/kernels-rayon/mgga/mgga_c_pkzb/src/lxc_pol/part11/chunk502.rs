//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 502/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk502(t12: f64, t24: f64, t1642: f64, t972: f64, t8: f64, t87: f64, t1429: f64, t439: f64, t1003: f64, t1651: f64, t91: f64, t507: f64, t98: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t84 = t12 <= zeta_threshold;
    let t90 = t24 <= zeta_threshold;
    let t2540 = t1642 * t972;
    let t2543 = t87 * t8;
    let t2547 = piecewise3(t84, 0.0_f64, 4.0_f64 / 9.0_f64 * t2540 * t439 + 8.0_f64 / 3.0_f64 * t2543 * t1429);
    let t2548 = t1651 * t1003;
    let t2551 = t91 * t8;
    let t2555 = piecewise3(t90, 0.0_f64, 4.0_f64 / 9.0_f64 * t2548 * t507 - 8.0_f64 / 3.0_f64 * t2551 * t1429);
    let t2557 = (t2547 + t2555) * t98;
    (t2540, t2548, t2557)
}
