//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2129/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2129(t27382: f64, t98633: f64, t198: f64, t206: f64, t7782: f64, t2: f64, t892: f64, t580: f64, t775: f64, t25206: f64, t1583: f64, t2430: f64) -> (f64, f64, f64, f64) {
    let t98635 = 2.0_f64 * t27382 * t98633;
    let t98637 = t198 * t206 * t7782;
    let t98646 = t892 * t2;
    let t98648 = t98646 * t580 * t775;
    let t98650 = 6.0_f64 * t25206 * t98648;
    let t98651 = t1583 * t2430;
    (t98635, t98637, t98650, t98651)
}
