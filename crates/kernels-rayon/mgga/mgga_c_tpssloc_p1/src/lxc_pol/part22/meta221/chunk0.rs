//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1268/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1268(t1458: f64, t88: f64, t1714: f64, t460: f64, t590: f64, t60: f64, t93: f64, t101: f64, t584: f64, t16: f64, t2: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7676 = t88 * t1458;
    let t8034 = t1714 * t460;
    let t8705 = 1.0_f64 / t60 / t590;
    let t9108 = t93 * t93;
    let t9174 = t101 * t101;
    let t9211 = 0.1044e2_f64 * t584;
    let t9212 = t2 * t16;
    (t7676, t8034, t8705, t9108, t9174, t9211, t9212)
}
