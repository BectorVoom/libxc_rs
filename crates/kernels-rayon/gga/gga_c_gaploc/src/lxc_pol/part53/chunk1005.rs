//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1005/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1005(t1456: f64, t1457: f64, t46941: f64, t1445: f64, t567: f64, t40374: f64, t40380: f64, t40397: f64, t40400: f64, t47877: f64, t587: f64, t912: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48066 = 0.35750489951850426669e0_f64 * t1456 * t1457 * t46941;
    let t48069 = 0.23005755572352449806e1_f64 * t567 * t1445 * t46941;
    let t48071 = 0.38342925953920749677e0_f64 * t40374;
    let t48073 = 0.51123901271894332903e0_f64 * t40380;
    let t48074 = 0.38342925953920749677e0_f64 * t40397;
    let t48076 = 0.76685851907841499354e0_f64 * t40400;
    let t48081 = t587 * t912 * t47877;
    (t48066, t48069, t48071, t48073, t48074, t48076, t48081)
}
