//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 953/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk953<F: Float>(t7045: F, t2271: F, t2810: F, t2813: F, t2452: F, t410: F, t372: F, t4845: F, t7025: F, t7028: F, t7031: F, t7033: F, t7036: F, t7039: F, t7043: F, t406: F) -> (F, F, F, F) {
    let t7046 = 6.0 * t7045;
    let t7048 = 0.4726e1 * t2271 * t2810;
    let t7050 = 0.4726e1 * t2271 * t2813;
    let t7051 = t410 * t2452;
    let t7052 = 8.0 * t7051;
    let t7053 = t372 * t7028 + t4845 - t7025 - t7031 - t7033 + t7036 - t7039 + t7043 - t7046 - t7048 - t7050 + t7052;
    let t7054 = t406 * t2452;
    (t7051, t7052, t7053, t7054)
}
