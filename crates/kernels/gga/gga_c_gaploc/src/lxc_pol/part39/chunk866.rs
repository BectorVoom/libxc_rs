//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 866/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk866<F: Float>(t4167: F, t883: F, t900: F, t1: F, t30795: F, t544: F, t10525: F, t2365: F, t30136: F, t12541: F, t7014: F, t2464: F, t2465: F, t2487: F, t9193: F) -> (F, F, F, F) {
    let t40165 = t883 * t4167;
    let t40166 = t900 * t40165;
    let t40167 = t544 * t30795 * t1 * t40166;
    let t40170 = t10525 * t2365 * t30136;
    let t40172 = t7014 * t12541;
    let t40176 = t2487 * t2464 * t2465 * t9193;
    (t40167, t40170, t40172, t40176)
}
