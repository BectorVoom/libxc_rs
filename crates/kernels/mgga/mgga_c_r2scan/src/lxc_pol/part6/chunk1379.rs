//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1379/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1379<F: Float>(t24442: F, t2550: F, t494: F, t6194: F, t2574: F, t3433: F, t20338: F, t2236: F, t7387: F, t6165: F, t6398: F, t8160: F, t146: F, t20946: F, t252: F, t1543: F, t2567: F) -> (F, F, F, F, F, F) {
    let t26174 = t24442 * t2550 * t494 * t6194;
    let t26175 = 0.43371823197556470519e-3 * t26174;
    let t26176 = t3433 * t2574;
    let t26177 = t20338 * t26176;
    let t26178 = 0.19043987679069580388e-1 * t26177;
    let t26179 = t2236 * t7387;
    let t26180 = 0.12713391885412927226e1 * t26179;
    let t26182 = t6165 * t6398 * t8160;
    let t26183 = 0.6112917064160653851e0 * t26182;
    let t26185 = t146 * t20946 * t252;
    let t26186 = t2567 * t1543;
    (t26175, t26178, t26180, t26183, t26185, t26186)
}
