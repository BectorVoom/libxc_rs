//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 876/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk876<F: Float>(t2801: F, t39337: F, t14289: F, t501: F, t605: F, t3718: F, t8042: F, t1382: F, t14348: F, t1052: F, t12270: F, t14350: F, t14408: F, t1960: F, t46004: F, t46006: F, t46008: F, t46013: F, t46016: F, t46019: F, t46023: F, t46025: F, t46832: F, t46835: F, t50781: F, t5559: F, t841: F) -> (F, F, F, F, F) {
    let t50789 = 4.0 * t39337 * t2801;
    let t50790 = t14289 * t501;
    let t50791 = t50790 * t605;
    let t50796 = 2.0 * t8042 * t3718;
    let t50799 = 2.0 * t1382 * t14348 * t605;
    let t50800 = 4.0 * t1052 * t12270 * t1960 - 12.0 * t14350 * t5559 * t841 + 2.0 * t14408 * t1960 * t841 + t46004 - t46006 + t46008 + t46013 - t46016 + t46019 - t46023 - t46025 + t46832 - t46835 + t50781 - t50789 + t50791 + t50796 - t50799;
    (t50789, t50791, t50796, t50799, t50800)
}
