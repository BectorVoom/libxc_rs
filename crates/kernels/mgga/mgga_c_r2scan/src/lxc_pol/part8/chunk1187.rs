//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1187/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1187<F: Float>(t1567: F, t1605: F, t122: F, t2304: F, t57: F, t6327: F, t6331: F, t108: F, t1541: F, t1607: F, t20954: F, t110: F, t144: F, t5132: F, t548: F, t2132: F, t6527: F) -> (F, F, F, F, F, F) {
    let t22962 = t1605 * t1567;
    let t22970 = 0.44555392061703122648e-3 * t6327 * t122 * t2304 * t57 * t6331;
    let t22980 = t1541 * t108;
    let t22985 = t20954 * t1607;
    let t22997 = t108 / t5132 / t144 * t110 * t548;
    let t23007 = t6527 * t2132;
    (t22962, t22970, t22980, t22985, t22997, t23007)
}
