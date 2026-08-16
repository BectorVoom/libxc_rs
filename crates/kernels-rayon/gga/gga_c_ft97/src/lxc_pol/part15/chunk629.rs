//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 629/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk629(t312: f64, t9577: f64, t1225: f64, t8232: f64, t309: f64, t799: f64, t1526: f64, t4406: f64, t7705: f64, t339: f64, t39: f64, t11: f64, t340: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15402 = t312 * t9577;
    let t15420 = t8232 * t1225;
    let t15460 = t799 * t309;
    let t15562 = t1526 * t7705 * t4406;
    let t15564 = t339 * t39;
    let t15565 = t340 * t11;
    (t15402, t15420, t15460, t15562, t15564, t15565)
}
