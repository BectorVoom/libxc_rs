//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 986/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk986(t1882: f64, t21970: f64, t21966: f64, t21953: f64, t41962: f64, t89: f64, t22162: f64, t375: f64, t21959: f64, t21950: f64, t21993: f64, t21979: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t83720 = t1882 * t21970;
    let t83722 = t1882 * t21966;
    let t83728 = t89 * t41962 * t21953;
    let t83770 = t89 * t375 * t22162;
    let t83772 = t1882 * t21959;
    let t83781 = t1882 * t21950;
    let t83789 = t1882 * t21993;
    let t83792 = t89 * t375 * t21979;
    (t83720, t83722, t83728, t83770, t83772, t83781, t83789, t83792)
}
