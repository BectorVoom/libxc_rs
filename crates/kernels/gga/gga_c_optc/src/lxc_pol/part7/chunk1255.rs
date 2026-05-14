//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1255/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1255<F: Float>(t1162: F, t2367: F, t9045: F, t3244: F, t9142: F, t9197: F, t9205: F, t1150: F, t3224: F, t7274: F, t3200: F, t3212: F, t3213: F, t3137: F, t3186: F, t3188: F) -> (F, F, F, F, F, F) {
    let t27755 = t1162 * t2367 * t9045;
    let t27758 = t3244 * t9142 * t9197;
    let t27761 = t1162 * t2367 * t9205;
    let t27768 = t1150 * t7274 * t3224;
    let t27771 = t3212 * t3200 * t3213;
    let t27778 = t3186 * t3137 * t3188;
    (t27755, t27758, t27761, t27768, t27771, t27778)
}
