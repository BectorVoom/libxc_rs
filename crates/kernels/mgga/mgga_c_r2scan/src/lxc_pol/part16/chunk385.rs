//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 385/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk385<F: Float>(t1691: F, t225: F, t704: F, t61: F, t732: F, t745: F, t1419: F, t230: F, t1422: F, t717: F, t720: F, t424: F, t697: F, t1678: F, t614: F, t22: F, t263: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1693 = t704 * t1691 * t225;
    let t1695 = 0.1301229756036208781e0 * t61 * t1693;
    let t1699 = t732 * t745;
    let t1702 = 12.0 * t1419 * t230;
    let t1704 = 32.0 * t1422 * t230;
    let t1706 = t717 * t1691;
    let t1707 = t1706 * t720;
    let t1709 = 0.19263893255070628431e1 * t61 * t1707;
    let t1710 = t424 * t697;
    let t1712 = t1678 * t614;
    let t1713 = t22 * t263;
    (t1693, t1695, t1699, t1702, t1704, t1707, t1709, t1710, t1712, t1713)
}
