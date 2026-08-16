//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 490/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk490(t221: f64, t2966: f64, t339: f64, t135: f64, t976: f64, t271: f64, t883: f64) -> (f64, f64, f64) {
    let t2967 = t221 * t2966;
    let t2969 = 0.18518518518518518518e-3_f64 * t339 * t2967;
    let t2970 = t135 * t976;
    let t2978 = 1.0_f64 / t271 / t883;
    (t2969, t2970, t2978)
}
