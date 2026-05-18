//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 390/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk390<F: Float>(t1691: F, t225: F, t704: F, t61: F, t732: F, t745: F, t1419: F, t230: F, t1422: F, t717: F, t720: F, t424: F, t697: F) -> (F, F, F, F, F, F, F, F) {
    let t1693 = t704 * t1691 * t225;
    let t1695 = F::new(0.1301229756036208781e0) * t61 * t1693;
    let t1699 = t732 * t745;
    let t1702 = F::new(12.0) * t1419 * t230;
    let t1704 = F::new(32.0) * t1422 * t230;
    let t1706 = t717 * t1691;
    let t1707 = t1706 * t720;
    let t1709 = F::new(0.19263893255070628431e1) * t61 * t1707;
    let t1710 = t424 * t697;
    (t1693, t1695, t1699, t1702, t1704, t1707, t1709, t1710)
}
