//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1337/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1337<F: Float>(t113971: F, t19968: F, t33437: F, t33373: F, t33469: F, t110635: F, t114205: F, t114231: F, t114455: F, t114464: F, t114493: F, t114499: F, t119174: F, t119298: F, t32019: F, t32022: F, t32096: F, t33588: F, t34715: F, t34744: F, t9449: F, t9809: F) -> (F, F) {
    let t119446 = t113971 * t19968 * t33437;
    let t119463 = t33373 * t33469;
    let t119465 = 0.44218518518518518516e-2 * t114455 - 0.11054629629629629629e-2 * t114464 - 0.77602083333333333337e-3 * t110635 * t119298 + 0.46561250000000000002e-2 * t110635 * t119446 + 0.20833333333333333334e-1 * t114205 * t9809 + 0.20833333333333333334e-1 * t114231 * t9809 + 0.20833333333333333334e-1 * t33373 * t33588 - t114493 - t114499 - 0.34722222222222222223e-2 * t119174 * t9449 + 0.69444444444444444446e-2 * t32096 * t34715 + 0.69444444444444444446e-2 * t32019 * t34715 + 0.55555555555555555557e-1 * t32022 * t34744 - 0.23148148148148148149e-2 * t119463;
    (t119446, t119465)
}
