//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 888/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk888(t11701: f64, t2728: f64, t11556: f64, t2355: f64, t1991: f64, t44707: f64, t590: f64, t739: f64, t1890: f64, t1966: f64, t44712: f64, t43370: f64) -> (f64, f64, f64, f64, f64) {
    let t45163 = t11701 * t2728;
    let t45164 = t2355 * t11556;
    let t45170 = 0.20449560508757733161e1_f64 * t1991 * t739 * t44707 * t590;
    let t45174 = 0.25561950635947166451e1_f64 * t1966 * t1890 * t44712 * t590;
    let t45176 = 0.23005755572352449806e1_f64 * t43370;
    (t45163, t45164, t45170, t45174, t45176)
}
