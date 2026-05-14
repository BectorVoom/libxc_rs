//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 649/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk649<F: Float>(t50: F, t5262: F, t5478: F, t4573: F, t5239: F, t38: F, t620: F, t22: F, t34: F, t39: F, t88: F, t35: F, t543: F, t2041: F, t538: F, t36: F, t1872: F, t539: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t51 = t50 <= zeta_threshold;
    let t5479 = t5262 + t5478;
    let t5483 = piecewise3(t51, 0.0, t4573);
    let t6116 = t5239 * rho1;
    let t6163 = t38 * t620;
    let t6165 = 1.0 / t22 / t6163;
    let t6316 = t34 * t39;
    let t6318 = 24.0 * t6316 * t88;
    let t6319 = t35 * t543;
    let t6320 = t6319 * t88;
    let t6321 = 144.0 * t6320;
    let t6322 = t538 * t2041;
    let t6323 = t6322 * t88;
    let t6324 = 240.0 * t6323;
    let t6325 = 1.0 / t6163;
    let t6326 = t36 * t6325;
    let t6328 = 120.0 * t6326 * t88;
    let t6329 = t539 * t1872;
    (t5479, t5483, t6116, t6165, t6316, t6318, t6319, t6321, t6322, t6324, t6325, t6326, t6328, t6329)
}
