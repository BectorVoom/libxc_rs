//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 569/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk569<F: Float>(t11318: F, t447: F, t1445: F, t2754: F, t2778: F, t11173: F, t11167: F, t475: F, t11255: F, t10308: F, t10312: F, t11309: F, t11312: F, t11315: F, t1450: F, t1562: F, t2386: F, t3566: F, t567: F, t574: F, t597: F) -> (F,) {
    let t11319 = t11318 * t447;
    let t11320 = t1445 * t11319;
    let t11323 = t2778 * t2754;
    let t11324 = t1445 * t11323;
    let t11327 = t1445 * t11173;
    let t11334 = t11167 * t475;
    let t11335 = t1445 * t11334;
    let t11338 = t1445 * t11255;
    let t11341 = -0.13803453343411469884e2 * t1562 * t11309 + 0.23005755572352449806e2 * t597 * t11312 + 0.23005755572352449806e1 * t567 * t11315 + 0.46011511144704899612e1 * t567 * t11320 - 0.92023022289409799224e1 * t574 * t11324 - 0.11502877786176224903e2 * t1450 * t11327 - 0.59584149919750711116e-1 * t10308 - 0.59584149919750711116e-1 * t10312 - 0.25025342966295298669e1 * t3566 * t2386 - 0.46011511144704899612e1 * t574 * t11335 + 0.11502877786176224903e2 * t597 * t11338;
    (t11341,)
}
