//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 822/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk822<F: Float>(t5495: F, t1046: F, t1750: F, t5513: F, t1817: F, t2612: F, t1006: F, t1740: F, t1033: F, t1778: F, t7280: F, t5042: F, t5083: F, t5085: F, t5087: F, t5360: F, t7233: F, t7249: F, t7252: F, t7255: F, t7259: F, t7262: F, t7266: F, t7269: F, t7272: F, t7278: F, t7285: F) -> (F, F, F, F, F, F, F) {
    let t7803 = 16.0 / 45.0 * t5495;
    let t7805 = 2.0 / 15.0 * t1750 * t1046;
    let t7806 = 8.0 / 135.0 * t5513;
    let t7808 = 8.0 / 45.0 * t2612 * t1817;
    let t7810 = 8.0 / 45.0 * t1006 * t1740;
    let t7811 = t1033 * t1778;
    let t7812 = 4.0 / 135.0 * t7811;
    let t7819 = 0.2518888888888888889e-2 * t7280;
    let t7829 = t5360 + 0.16792592592592592593e-2 * t5083 - 0.41981481481481481482e-3 * t5087 + 0.12594444444444444445e-2 * t5042 - 0.62972222222222222223e-3 * t5085 + 0.83962962962962962964e-3 * t7269 - 0.83962962962962962965e-3 * t7278 + t7819 + 0.1385388888888888889e-1 * t7272 + 0.20990740740740740742e-2 * t7285 - 0.75566666666666666669e-2 * t7249 - 0.50377777777777777779e-2 * t7252 + 0.12594444444444444445e-2 * t7255 + 0.11335e-1 * t7259 + 0.15113333333333333334e-1 * t7262 - 0.37783333333333333334e-2 * t7266 - 0.37783333333333333334e-2 * t7233;
    (t7803, t7805, t7806, t7808, t7810, t7812, t7829)
}
