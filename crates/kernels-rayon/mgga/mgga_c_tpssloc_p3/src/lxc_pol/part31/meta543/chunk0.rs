//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1765/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1765(t22690: f64, t23153: f64, t23171: f64, t6561: f64, t80741: f64, t6643: f64, t23025: f64, t23030: f64, t23012: f64, t6653: f64, t22641: f64, t2588: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81595 = t23171 * t22690 * t23153;
    let t81597 = t80741 * t6561;
    let t81598 = t81597 * t6643;
    let t81600 = t23030 * t23025;
    let t81602 = t23012 * t6653;
    let t81612 = t22641 * t2588;
    (t81595, t81597, t81598, t81600, t81602, t81612)
}
