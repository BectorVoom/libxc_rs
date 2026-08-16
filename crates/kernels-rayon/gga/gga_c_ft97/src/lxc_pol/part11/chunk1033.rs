//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1033/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1033(t235: f64, t9680: f64, t226: f64, t2428: f64, t2393: f64, t688: f64, t13521: f64, t2455: f64, t709: f64, t9548: f64, t2395: f64, t2417: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41547 = 1.0_f64 / t9680 / t235;
    let t41548 = t226 * t41547;
    let t41549 = t2428 * t2428;
    let t41557 = t2393 * t688;
    let t41561 = t13521 * t2455;
    let t41569 = t2455 * t2455;
    let t41573 = t9548 * t709;
    let t41577 = t2395 * t2417;
    (t41548, t41549, t41557, t41561, t41569, t41573, t41577)
}
