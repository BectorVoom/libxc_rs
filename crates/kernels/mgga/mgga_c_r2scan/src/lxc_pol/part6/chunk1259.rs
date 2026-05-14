//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1259/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1259<F: Float>(t18858: F, t18860: F, t18862: F, t18865: F, t18879: F, t18882: F, t18884: F, t18869: F, t18872: F, t18875: F, t18878: F, t18888: F, t18889: F, t18891: F, t18894: F, t18896: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t23696 = 4.0 * t18858;
    let t23697 = 0.17544670867903938621e1 * t18860;
    let t23698 = 0.51947577317044391277e2 * t18862;
    let t23699 = 0.97592231702715658578e-1 * t18865;
    let t23700 = 24.0 * t18879;
    let t23701 = 3.0 * t18882;
    let t23702 = 0.14447919941302971324e1 * t18884;
    let t23703 = t23696 + t23697 + t23698 + t23699 + t18869 - t18872 - t18875 - t18878 + t23700 - t23701 - t23702 - t18888;
    let t23705 = 0.32530743900905219526e-1 * t18889;
    let t23706 = 0.48796115851357829289e-1 * t18891;
    let t23707 = 3.0 * t18894;
    let t23708 = 480.0 * t18896;
    (t23696, t23697, t23698, t23699, t23700, t23701, t23702, t23703, t23705, t23706, t23707, t23708)
}
