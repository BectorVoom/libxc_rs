//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1847/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1847(t20553: f64, t550: f64, t1343: f64, t820: f64, t1799: f64, t6347: f64, t3870: f64, t20489: f64, t20416: f64, t210: f64, t214: f64, t20356: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20554 = t20553 * t550;
    let t20556 = t1343 * t820 * t20554;
    let t20563 = t1799 * t6347;
    let t20565 = t3870 * t820 * t20563;
    let t20568 = t20489 * t550;
    let t20570 = t1343 * t820 * t20568;
    let t20576 = t210 * t214 * t20416;
    let t20582 = t210 * t214 * t20356;
    (t20554, t20556, t20563, t20565, t20568, t20570, t20576, t20582)
}
