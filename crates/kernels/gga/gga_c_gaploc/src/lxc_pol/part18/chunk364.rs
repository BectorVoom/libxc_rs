//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 364/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk364<F: Float>(t293: F, t711: F, t291: F, t1233: F, t1236: F, t1685: F, t1227: F, t286: F, t458: F, t714: F, t1237: F, t1681: F, t1687: F, t295: F, t471: F, t64: F, t719: F, t90: F) -> (F, F, F, F, F) {
    let t1691 = 1.0 / t711 / t293;
    let t1692 = t291 * t1691;
    let t1693 = t1692 * t1233;
    let t1695 = t1236 * t1685 * M_PI;
    let t1699 = t1227 * t286 * t458;
    let t1700 = t714 * t1699;
    let t1702 = 63.0 / 256.0 * t1681 - 49.0 / 8192.0 * t1237 * t1687 + 49.0 / 24576.0 * t1693 * t1695 - 21.0 / 256.0 * t1700;
    let t1710 = t1702 * t471 - 4.0 / 3.0 * t719 * t64 + 7.0 / 96.0 * t1681 - 7.0 / 288.0 * t1700 + 4.0 / 3.0 * t295 * t90;
    (t1691, t1692, t1695, t1699, t1710)
}
