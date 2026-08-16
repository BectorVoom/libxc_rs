//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 967/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk967(t2018: f64, t26161: f64, t3698: f64, t92169: f64, t31338: f64, t81651: f64, t82074: f64, t1888: f64, t23270: f64, t26728: f64, t2719: f64, t1880: f64, t23196: f64, t31366: f64) -> (f64, f64, f64, f64) {
    let t114573 = 6.0_f64 * t26161 * t92169 * t2018 * t3698;
    let t114592 = t81651 * t82074 * t31338;
    let t114596 = t1888 * t23270 * t26728 * t2719;
    let t114599 = t1880 * t31366 * t23196;
    (t114573, t114592, t114596, t114599)
}
