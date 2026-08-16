//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2379/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2379(t2694: f64, t9784: f64, t16: f64, t2236: f64, t240: f64, t236: f64, t243: f64, t281: f64, t39644: f64, t10871: f64, t775: f64, t10696: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40639 = t9784 * t2694;
    let t40648 = t2236 * t16;
    let t40649 = 1.0_f64 / t40648;
    let t40650 = t40649 * t240;
    let t40654 = 0.47607864835161149081e-7_f64 * t39644 * t236 * t40650 * t243 * t281;
    let t40664 = t10871 * t775;
    let t40672 = t10696 * t72;
    (t40639, t40649, t40650, t40654, t40664, t40672)
}
