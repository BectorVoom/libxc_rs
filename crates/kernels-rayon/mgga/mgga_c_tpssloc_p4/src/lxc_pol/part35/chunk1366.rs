//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1366/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1366(t25224: f64, t28276: f64, t6552: f64, t1484: f64, t23270: f64, t25038: f64, t98169: f64, t20800: f64, t6553: f64, t6554: f64, t1880: f64, t28294: f64) -> (f64, f64, f64, f64) {
    let t105445 = t6552 * t25224 * t28276;
    let t105449 = t25038 * t23270 * t98169 * t1484;
    let t105453 = t6552 * t6553 * t6554 * t20800;
    let t105462 = t1880 * t25224 * t28294;
    (t105445, t105449, t105453, t105462)
}
