//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1254/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1254(t7604: f64, t82632: f64, t1920: f64, t2966: f64, t7614: f64, t7607: f64, t23518: f64, t7577: f64, t7561: f64, t7557: f64, t11094: f64, t7627: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t89366 = t82632 * t7604;
    let t89431 = t1920 * t2966 * t7614;
    let t89449 = t82632 * t7607;
    let t89473 = t7577 * t23518;
    let t89617 = t1920 * t2966 * t7561;
    let t89672 = t82632 * t7557;
    let t89702 = t7627 * t11094;
    (t89366, t89431, t89449, t89473, t89617, t89672, t89702)
}
