//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1146/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1146(t27948: f64, t67: f64, t1864: f64, t7441: f64, t7445: f64, t5441: f64, t71: f64, t1863: f64, t5389: f64, t79: f64, t72: f64, t1410: f64, t3953: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27949 = t27948 * t67;
    let t27950 = t27949 * t1864;
    let t27953 = t7441 * t7445;
    let t27956 = t71 * t5441;
    let t27957 = t1863 * t27956;
    let t27960 = t79 * t5389;
    let t27961 = t72 * t27960;
    let t27966 = t3953 * t1410;
    (t27949, t27950, t27953, t27956, t27957, t27961, t27966)
}
