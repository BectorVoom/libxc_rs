//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1042/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1042(t2347: f64, t41468: f64, t2404: f64, t92: f64, t41473: f64, t1771: f64, t2410: f64, t458: f64, t9579: f64, t9584: f64, t9588: f64, t9593: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41726 = t2347 * t41468;
    let t41728 = t92 * t2404 * t41726;
    let t41731 = t92 * t2404 * t41473;
    let t41733 = t1771 * t2410;
    let t41735 = t458 * t9579;
    let t41737 = t458 * t9584;
    let t41739 = t458 * t9588;
    let t41741 = t458 * t9593;
    (t41726, t41728, t41731, t41733, t41735, t41737, t41739, t41741)
}
