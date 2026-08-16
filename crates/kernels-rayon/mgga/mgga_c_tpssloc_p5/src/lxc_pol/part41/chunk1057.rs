//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1057/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1057(t232: f64, t4119: f64, t2645: f64, t4181: f64, t16891: f64, t2647: f64, t13242: f64, t5591: f64, t13228: f64, t13351: f64, t13222: f64, t16839: f64, t9627: f64) -> (f64, f64, f64, f64, f64) {
    let t16912 = t232 * t4119;
    let t16914 = t2645 * t4181 * t16912;
    let t16918 = t2645 * t16891 * t2647;
    let t16924 = t2645 * t13242 * t5591;
    let t16927 = t13228 * t13351;
    let t16928 = t13222 * t16927;
    let t16932 = t2645 * t16839 * t9627;
    (t16914, t16918, t16924, t16928, t16932)
}
