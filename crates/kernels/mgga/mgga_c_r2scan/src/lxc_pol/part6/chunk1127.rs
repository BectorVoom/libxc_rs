//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1127/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1127<F: Float>(t20434: F, t6214: F, t1551: F, t6212: F, t6209: F, t6211: F, t122: F, t2111: F, t2117: F, t4888: F, t57: F, t6065: F, t6407: F, t409: F, t5: F, t511: F, t7: F) -> (F, F, F, F, F) {
    let t20435 = t20434 * t6214;
    let t20437 = t6212 * t1551;
    let t20439 = t6209 * t6211 * t20437;
    let t20445 = 0.14714292610726565554e-1 * t2111 * t122 * t4888 * t57 * t2117;
    let t20446 = t6407 * t6065;
    let t20450 = t5 * t7 * t409 * t511;
    (t20435, t20439, t20445, t20446, t20450)
}
