//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 852/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk852<F: Float>(t22507: F, t83: F, t22504: F, t22509: F, t1332: F, t1755: F, t452: F, t488: F, t1647: F, t5630: F, t1902: F, t1882: F, t5724: F, t1339: F, t1588: F, t1871: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23158 = t83 * t22507;
    let t23161 = t83 * t22504;
    let t23164 = t83 * t22509;
    let t23167 = t1332 * t1755;
    let t23169 = t452 * t488 * t23167;
    let t23172 = t5630 * t1647;
    let t23173 = t1902 * t23172;
    let t23176 = t1882 * t5724;
    let t23179 = t1871 * t1339 * t1588;
    (t23158, t23161, t23164, t23167, t23169, t23172, t23173, t23176, t23179)
}
