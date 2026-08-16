//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 890/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk890(t6121: f64, t8820: f64, t360: f64, t1569: f64, t2530: f64, t2572: f64, t2124: f64, t9317: f64, t2590: f64, t259: f64, t8196: f64, t571: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9501 = t8820 * t6121;
    let t9502 = t360 * t9501;
    let t9507 = t1569 * t2530;
    let t9508 = t2572 * t9507;
    let t9509 = t360 * t9508;
    let t9513 = t2124 * t9317 * t6121;
    let t9517 = t2124 * t2590 * t9507;
    let t9520 = t8196 * t259;
    let t9521 = t571 * t9520;
    (t9501, t9502, t9508, t9509, t9513, t9517, t9521)
}
