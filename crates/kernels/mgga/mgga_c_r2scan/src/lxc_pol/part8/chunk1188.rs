//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1188/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1188<F: Float>(t19092: F, t507: F, t512: F, t119: F, t122: F, t19091: F, t1559: F, t283: F, t114: F, t22671: F, t6817: F, t2054: F, t22666: F, t22644: F, t758: F, t2298: F) -> (F, F, F, F, F, F, F) {
    let t23018 = 0.11535463037670312369e1 * t512 * t19092 * t507;
    let t23025 = 0.64878869728179484639e0 * t19091 * t119 * t122 * t507;
    let t23038 = t1559 * t1559;
    let t23040 = 1.0 / t283 / t23038;
    let t23041 = t114 * t23040;
    let t23059 = t6817 * t22671;
    let t23063 = t2054 * t22666;
    let t23067 = t758 * t22644;
    let t23099 = t2298 * t2298;
    (t23018, t23025, t23041, t23059, t23063, t23067, t23099)
}
