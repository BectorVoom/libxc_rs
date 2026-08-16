//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 935/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk935(t1935: f64, t29575: f64, t24473: f64, t2580: f64, t2586: f64, t9085: f64, t741: f64, t29542: f64, t29545: f64, t29548: f64, t29551: f64, t29554: f64, t29556: f64, t29558: f64, t29562: f64, t29565: f64, t29567: f64, t29569: f64, t29573: f64) -> (f64, f64, f64, f64) {
    let t29576 = t1935 * t29575;
    let t29578 = t24473 * t2580;
    let t29580 = t2586 * t9085;
    let t29581 = t741 * t29580;
    let t29583 = -t29542 / 24.0_f64 + 19.0_f64 / 48.0_f64 * t29545 + t29548 / 64.0_f64 + 3.0_f64 / 8.0_f64 * t29551 - t29554 / 192.0_f64 + 3.0_f64 / 8.0_f64 * t29556 - 3.0_f64 / 16.0_f64 * t29558 - t29562 / 192.0_f64 - 19.0_f64 / 36.0_f64 * t29565 - t29567 / 64.0_f64 + t29569 / 8.0_f64 + t29573 / 24.0_f64 + t29576 / 2.0_f64 + 3.0_f64 / 256.0_f64 * t29578 + t29581 / 12.0_f64;
    (t29576, t29578, t29581, t29583)
}
