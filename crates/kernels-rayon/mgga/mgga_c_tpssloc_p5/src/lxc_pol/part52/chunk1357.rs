//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1357/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1357(t31058: f64, t7458: f64, t652: f64, t6534: f64, t7670: f64, t19456: f64, t8327: f64, t4028: f64, t1976: f64, t26135: f64, t12725: f64, t25010: f64, t8450: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t120721 = 2.0_f64 * t7458 * t31058;
    let t120723 = t652 * t7670 * t6534;
    let t120728 = 2.0_f64 * t19456 * t8327;
    let t120730 = 2.0_f64 * t4028 * t31058;
    let t120732 = t652 * t1976 * t26135;
    let t120735 = 2.0_f64 * t12725 * t8327;
    let t120738 = t8450 * t25010;
    (t120721, t120723, t120728, t120730, t120732, t120735, t120738)
}
