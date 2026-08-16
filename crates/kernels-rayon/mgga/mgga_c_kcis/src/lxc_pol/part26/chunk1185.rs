//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1185/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1185(t1363: f64, t21453: f64, t1494: f64, t6927: f64, t4134: f64, t7202: f64, t3960: f64, t7028: f64, t1628: f64, t23253: f64, t286: f64, t69: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t60299 = t21453 * t1363;
    let t60756 = t1494 * t6927;
    let t60761 = t4134 * t7202;
    let t60780 = t7028 * t3960;
    let t60988 = t23253 * t1628;
    let t61287 = t69 * t286;
    (t60299, t60756, t60761, t60780, t60988, t61287)
}
