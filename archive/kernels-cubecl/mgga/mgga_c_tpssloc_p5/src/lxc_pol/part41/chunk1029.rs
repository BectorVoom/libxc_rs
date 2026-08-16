//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1029/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1029<F: Float>(t1336: F, t16397: F, t5252: F, t225: F, t5319: F, t5217: F, t1390: F, t5356: F, t112: F, t5363: F, t111: F, t1851: F) -> (F, F, F, F, F, F) {
    let t16398 = t1336 * t16397;
    let t16400 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t16398 * t5252;
    let t16439 = t5319 * t225;
    let t16460 = t5217 * t225;
    let t16497 = t5356 * t1390;
    let t16521 = t5363 * t112;
    let t16524 = t1851 * t111;
    (t16400, t16439, t16460, t16497, t16521, t16524)
}
