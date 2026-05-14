//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1236/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1236<F: Float>(t6055: F, t780: F, t1567: F, t1605: F, t20306: F, t2155: F, t122: F, t2304: F, t57: F, t6327: F, t6331: F, t20343: F, t6395: F, t6542: F, t108: F, t1541: F) -> (F, F, F, F, F, F, F) {
    let t22960 = t6055 * t780;
    let t22962 = t1605 * t1567;
    let t22964 = t2155 * t22962 * t20306;
    let t22970 = 0.44555392061703122648e-3 * t6327 * t122 * t2304 * t57 * t6331;
    let t22975 = t2155 * t20343;
    let t22978 = t6395 * t6542;
    let t22980 = t1541 * t108;
    (t22960, t22962, t22964, t22970, t22975, t22978, t22980)
}
