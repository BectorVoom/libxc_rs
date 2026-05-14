//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1299/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1299<F: Float>(t2651: F, t8165: F, t1591: F, t9463: F, t2620: F, t7566: F, t255: F, t537: F, t571: F, t9083: F, t1604: F, t30050: F, t2207: F, t785: F, t788: F, t8629: F) -> (F, F, F, F, F, F) {
    let t31030 = t2651 * t8165;
    let t31037 = t1591 * t9463;
    let t31040 = t7566 * t2620;
    let t31044 = t571 * t537 * t9083 * t255;
    let t31047 = t1604 * t30050;
    let t31051 = t2207 * t785 * t788 * t8629;
    (t31030, t31037, t31040, t31044, t31047, t31051)
}
