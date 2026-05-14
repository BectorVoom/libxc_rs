//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 450/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk450<F: Float>(t1693: F, t61: F, t732: F, t745: F, t1419: F, t230: F, t1422: F, t1691: F, t717: F, t720: F) -> (F, F, F, F, F, F) {
    let t1695 = 0.1301229756036208781e0 * t61 * t1693;
    let t1699 = t732 * t745;
    let t1702 = 12.0 * t1419 * t230;
    let t1704 = 32.0 * t1422 * t230;
    let t1706 = t717 * t1691;
    let t1707 = t1706 * t720;
    (t1695, t1699, t1702, t1704, t1706, t1707)
}
