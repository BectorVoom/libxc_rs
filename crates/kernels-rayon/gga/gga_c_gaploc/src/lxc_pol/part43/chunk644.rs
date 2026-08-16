//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 644/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk644(t11986: f64, t475: f64, t1445: f64, t11982: f64, t11987: f64, t11977: f64, t188: f64, t1457: f64, t3701: f64, t528: f64, t1: f64, t3689: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12044 = t11986 * t475;
    let t12045 = t1445 * t12044;
    let t12048 = t1445 * t11982;
    let t12051 = t1445 * t11987;
    let t12054 = t188 * t11977;
    let t12057 = t1457 * t11982;
    let t12060 = t528 * t3701;
    let t12063 = t3689 * t1;
    (t12045, t12048, t12051, t12054, t12057, t12060, t12063)
}
