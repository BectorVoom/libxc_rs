//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1146/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1146(t1012: f64, t1014: f64, t1017: f64, t1030: f64, t3053: f64, t30840: f64, t3068: f64, t30827: f64, t23448: f64, t8384: f64, t23442: f64, t1036: f64, t30833: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t113397 = t1012 * t1014 * t1030 * t1017;
    let t113400 = t30840 * t3053;
    let t113413 = t1012 * t30827 * t3068;
    let t113416 = t23448 * t8384;
    let t113418 = t23442 * t8384;
    let t113429 = t30833 * t1036;
    (t113397, t113400, t113413, t113416, t113418, t113429)
}
