//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 758/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk758(t11218: f64, t203: f64, t599: f64, t3529: f64, t447: f64, t2366: f64, t1352: f64, t3530: f64, t3516: f64, t6508: f64, t1959: f64, t3634: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36117 = t203 * t11218;
    let t36178 = t599 * t11218;
    let t36210 = t3529 * t447;
    let t36211 = t2366 * t36210;
    let t36247 = t3530 * t1352;
    let t36273 = t3516 * t447;
    let t36274 = t6508 * t36273;
    let t36313 = t3634 * t1959;
    (t36117, t36178, t36211, t36247, t36274, t36313)
}
