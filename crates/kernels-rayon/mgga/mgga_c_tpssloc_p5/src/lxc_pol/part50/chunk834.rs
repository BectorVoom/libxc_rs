//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 834/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk834(t533: f64, t8492: f64, t3701: f64, t1983: f64, t113: f64, t1869: f64, t1976: f64, t510: f64, t574: f64, t8313: f64, t8315: f64, t8322: f64, t8324: f64, t8329: f64, t8439: f64, t8447: f64, t8451: f64, t8491: f64) -> (f64, f64, f64) {
    let t8493 = t533 * t8492;
    let t8494 = t8493 * t3701;
    let t8495 = t1983 * t8494;
    let t8496 = -t113 * t8439 - 2.0_f64 * t1869 * t1976 - t510 * t8313 + t574 * t8447 - 4.0_f64 * t8315 - t8322 - 4.0_f64 * t8324 - t8329 + 2.0_f64 * t8451 + t8491 - t8495;
    (t8493, t8494, t8496)
}
