//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 738/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk738<F: Float>(t44: F, t6897: F, t986: F, t1048: F, t2330: F, t1543: F, t2854: F, t2858: F, t4987: F, t4938: F, t889: F, t1361: F, t35: F, t1216: F, t415: F, t1213: F, t1219: F, t2466: F, t2469: F, t40: F, t48: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t6967 = t986 * t6897;
    let t6969 = t1048 * t6967 * t2330;
    let t6970 = 2.0 * t6969;
    let t6972 = t2858 * t2854 * t1543;
    let t6973 = 6.0 * t6972;
    let t6975 = 0.34631718211362927518e2 * t4987;
    let t6976 = t4938 * t889;
    let t6979 = t1361 * t35;
    let t6980 = t1216 * t415;
    let t6990 = piecewise3(t45, 0.0, -8.0 / 27.0 * t6976 * t1213 + 16.0 / 9.0 * t6979 * t6980 + 4.0 / 9.0 * t2466 * t1219 + 8.0 / 3.0 * t48 * t1216 - 8.0 * t2469 * t40);
    (t6967, t6970, t6973, t6975, t6980, t6990)
}
