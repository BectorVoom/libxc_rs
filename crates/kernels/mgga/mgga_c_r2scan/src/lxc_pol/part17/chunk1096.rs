//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1096/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1096<F: Float>(t2650: F, t546: F, t565: F, t10698: F, t2559: F, t10772: F, t10810: F, t2578: F, t1577: F, t2599: F, t2096: F, t2649: F, t571: F) -> (F, F, F, F, F, F) {
    let t39375 = t546 * t2650;
    let t39378 = t565 * t2650;
    let t39395 = t10698 * t2559;
    let t39400 = t10772 * t10810 * t2578;
    let t39403 = t1577 * t10810 * t2599;
    let t39409 = t571 * t2649 * t2096;
    (t39375, t39378, t39395, t39400, t39403, t39409)
}
