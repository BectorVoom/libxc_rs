//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 543/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk543(t200: f64, t6757: f64, t1113: f64, t203: f64, t237: f64, t6: f64, t1100: f64, t5011: f64, t1200: f64, t287: f64, t173: f64, t174: f64, rho1: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6758 = t6757 * t200;
    let t6762 = t203 * t1113;
    let t6783 = t237 * t6;
    let t6784 = t1100 * t6783;
    let t6816 = t5011 * rho1;
    let t7003 = t1200 * t287;
    let t7239 = t173 * t174;
    (t6758, t6762, t6783, t6784, t6816, t7003, t7239)
}
