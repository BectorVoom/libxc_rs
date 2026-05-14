//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 831/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk831<F: Float>(t609: F, t1599: F, t6168: F, t2104: F, t531: F, t833: F, t4440: F, t286: F, t494: F, t4455: F, t1610: F, t6136: F, t1608: F, t1603: F, t1612: F, t4424: F, t4427: F, t4430: F, t4439: F, t6138: F, t6141: F, t6149: F, t6152: F, t6156: F, t6160: F, t6165: F) -> (F, F, F, F, F, F, F, F) {
    let t614 = 0.0 < t609;
    let t6169 = t1599 * t6168;
    let t6171 = t2104 * t531;
    let t6172 = t6171 * t833;
    let t6173 = t4440 * t6172;
    let t6176 = t286 * t494;
    let t6177 = t4455 * t2104;
    let t6178 = t6177 * t1610;
    let t6179 = t6176 * t6178;
    let t6183 = piecewise3(t614, t6136, -t6136);
    let t6184 = t1608 * t6183;
    let t6185 = t286 * t6184;
    let t6188 = -t6138 / 216.0 - t6141 * t1603 / 216.0 + t6141 * t1612 / 72.0 - t4424 + t4427 / 1728.0 - t4430 / 576.0 + t6149 / 1728.0 + t4439 * t6152 / 432.0 - t4439 * t6156 / 576.0 - t4439 * t6160 / 288.0 - t1599 * t6165 / 288.0 - t6169 / 576.0 - t4439 * t6173 / 576.0 + t1599 * t6179 / 96.0 - t1599 * t6185 / 192.0;
    (t6171, t6172, t6176, t6177, t6178, t6183, t6184, t6188)
}
