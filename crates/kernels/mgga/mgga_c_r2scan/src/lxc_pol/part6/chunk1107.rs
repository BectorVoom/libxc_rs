//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1107/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1107<F: Float>(t19790: F, t481: F, t19787: F, t19789: F, t2201: F, t560: F, t6263: F, t785: F, t2207: F, t1591: F, t2597: F, t6148: F, t2132: F, t6448: F, t1620: F, t5120: F) -> (F, F, F, F, F, F, F) {
    let t19791 = t19790 * t481;
    let t19793 = t19787 * t19789 * t19791;
    let t19797 = t2201 * t785 * t6263 * t560;
    let t19801 = t2207 * t785 * t6263 * t481;
    let t19807 = t1591 * t2597;
    let t19820 = t1591 * t6148;
    let t19827 = t6448 * t2132;
    let t19837 = t1620 * t5120;
    (t19793, t19797, t19801, t19807, t19820, t19827, t19837)
}
