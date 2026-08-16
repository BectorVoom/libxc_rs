//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 630/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk630(t11318: f64, t447: f64, t1445: f64, t2754: f64, t2778: f64, t11173: f64, t11167: f64, t475: f64, t11255: f64, t10308: f64, t10312: f64, t11309: f64, t11312: f64, t11315: f64, t1450: f64, t1562: f64, t2386: f64, t3566: f64, t567: f64, t574: f64, t597: f64) -> f64 {
    let t11319 = t11318 * t447;
    let t11320 = t1445 * t11319;
    let t11323 = t2778 * t2754;
    let t11324 = t1445 * t11323;
    let t11327 = t1445 * t11173;
    let t11334 = t11167 * t475;
    let t11335 = t1445 * t11334;
    let t11338 = t1445 * t11255;
    let t11341 = -0.13803453343411469884e2_f64 * t1562 * t11309 + 0.23005755572352449806e2_f64 * t597 * t11312 + 0.23005755572352449806e1_f64 * t567 * t11315 + 0.46011511144704899612e1_f64 * t567 * t11320 - 0.92023022289409799224e1_f64 * t574 * t11324 - 0.11502877786176224903e2_f64 * t1450 * t11327 - 0.59584149919750711116e-1_f64 * t10308 - 0.59584149919750711116e-1_f64 * t10312 - 0.25025342966295298669e1_f64 * t3566 * t2386 - 0.46011511144704899612e1_f64 * t574 * t11335 + 0.11502877786176224903e2_f64 * t597 * t11338;
    t11341
}
