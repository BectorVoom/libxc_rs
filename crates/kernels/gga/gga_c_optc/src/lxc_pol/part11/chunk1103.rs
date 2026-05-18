//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1103/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1103<F: Float>(t11603: F, t5257: F, t36863: F, t5228: F, t4297: F, t190: F, t3086: F, t136: F, t1220: F, t5232: F, t7274: F, t23: F, t5236: F, t5238: F, t5241: F) -> (F, F, F, F, F) {
    let t43571 = t5257 * t11603;
    let t43583 = t36863 * t5228;
    let t43584 = t4297 * t43583;
    let t43635 = t3086 * t190;
    let t43636 = t43635 * t136;
    let t43649 = t1220 * t7274 * t5232;
    let t43671 = t5236 * t5238 * t5241 * t23;
    (t43571, t43584, t43636, t43649, t43671)
}
