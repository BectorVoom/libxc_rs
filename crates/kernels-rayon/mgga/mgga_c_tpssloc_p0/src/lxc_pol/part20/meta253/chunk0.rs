//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1381/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1381(t819: f64, t820: f64, t9981: f64, t2639: f64, t2686: f64, t2697: f64, t2703: f64, t842: f64, t9612: f64, t2617: f64, t2696: f64) -> (f64, f64, f64, f64, f64) {
    let t9983 = t819 * t820 * t9981;
    let t9986 = t2639 * t2686;
    let t9988 = t2697 * t2703;
    let t9990 = t9612 * t842;
    let t9993 = t2617 * t2696;
    (t9983, t9986, t9988, t9990, t9993)
}
