//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 969/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk969(t1066: f64, t154: f64, t5688: f64, t276: f64, t2048: f64, t2739: f64, t7350: f64, t742: f64, t2932: f64, t5974: f64, t2104: f64, t1885: f64, t287: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7620 = t154 * t5688 * t1066;
    let t7621 = t276 * t7620;
    let t7628 = t154 * t2048 * t2739;
    let t7630 = t276 * t7628 / 144.0_f64;
    let t7632 = t154 * t742 * t7350;
    let t7637 = t5974 * t2932;
    let t7639 = 0.57165357490759649296e-3_f64 * t2104 * t7637;
    let t7640 = t287 * t1885;
    (t7620, t7621, t7628, t7630, t7632, t7637, t7639, t7640)
}
