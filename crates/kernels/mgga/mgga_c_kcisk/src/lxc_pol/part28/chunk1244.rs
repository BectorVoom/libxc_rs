//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1244/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1244<F: Float>(t35325: F, t9704: F, t2559: F, t2568: F, t34316: F, t9977: F, t1873: F, t8972: F, t34313: F, t9972: F, t35309: F, t35311: F, t35314: F, t35317: F, t35319: F, t35321: F, t35323: F) -> (F, F, F, F, F, F) {
    let t35326 = t9704 * t35325;
    let t35328 = t2559 * t2568;
    let t35330 = t34316 * t9977;
    let t35332 = t1873 * t8972;
    let t35334 = t34313 * t9972;
    let t35336 = -2.0 / 9.0 * t35309 + t35311 / 16.0 - t35314 / 72.0 + t35317 / 24.0 - t35319 / 128.0 - 19.0 / 72.0 * t35321 + t35323 / 9.0 - t35326 / 16.0 - t35328 / 3.0 + t35330 / 12.0 + t35332 / 128.0 - t35334 / 8.0;
    (t35326, t35328, t35330, t35332, t35334, t35336)
}
