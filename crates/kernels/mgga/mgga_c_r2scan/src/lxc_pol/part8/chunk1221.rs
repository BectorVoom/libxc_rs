//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1221/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1221<F: Float>(t2583: F, t3433: F, t19851: F, t571: F, t2578: F, t19852: F, t24442: F, t2550: F, t494: F, t6194: F, t2574: F, t20338: F, t2236: F, t7387: F, t6165: F, t6398: F, t8160: F) -> (F, F, F, F, F, F) {
    let t26145 = t3433 * t2583;
    let t26146 = t571 * t19851 * t26145;
    let t26147 = 0.19043987679069580388e-1 * t26146;
    let t26150 = t3433 * t2578;
    let t26151 = t19852 * t26150;
    let t26174 = t24442 * t2550 * t494 * t6194;
    let t26175 = 0.43371823197556470519e-3 * t26174;
    let t26176 = t3433 * t2574;
    let t26177 = t20338 * t26176;
    let t26178 = 0.19043987679069580388e-1 * t26177;
    let t26179 = t2236 * t7387;
    let t26180 = 0.12713391885412927226e1 * t26179;
    let t26182 = t6165 * t6398 * t8160;
    (t26147, t26151, t26175, t26178, t26180, t26182)
}
