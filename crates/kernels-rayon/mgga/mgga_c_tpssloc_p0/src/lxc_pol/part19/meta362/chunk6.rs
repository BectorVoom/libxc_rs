//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1320/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1320(t10904: f64, t11002: f64, t10508: f64, t248: f64, t3130: f64, t3132: f64, t10969: f64, t121: f64, t10305: f64, t1041: f64, t1015: f64, t3033: f64, t42520: f64) -> (f64, f64, f64, f64) {
    let t42582 = t10904 * t11002;
    let t42586 = t3130 * t248 * t10508 * t3132;
    let t42592 = t121 * t10969;
    let t42595 = t1041 * t248 * t42592 * t10305;
    let t42600 = t3033 * t1015 * t42520;
    (t42582, t42586, t42595, t42600)
}
