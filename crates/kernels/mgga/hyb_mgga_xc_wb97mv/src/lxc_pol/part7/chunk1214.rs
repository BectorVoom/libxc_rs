//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1214/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1214<F: Float>(t10356: F, t1852: F, t10351: F, t10340: F, t10344: F, t8225: F, t10329: F, t10273: F, t10327: F, t10338: F, t10349: F, t10354: F, t1859: F, t1861: F, t1867: F, t1877: F, t3025: F, t3031: F, t3854: F, t544: F, t571: F, t6147: F, t8228: F) -> (F,) {
    let t29345 = t1852 * t10356;
    let t29347 = t1852 * t10351;
    let t29353 = t1852 * t10340;
    let t29355 = t8225 * t10344;
    let t29361 = t1852 * t10329;
    let t29363 = t571 * t3031 * t10354 * t1877 / 27.0 - 2.0 / 81.0 * t571 * t3025 * t1859 * t10273 * t544 - t571 * t3025 * t10349 * t1877 / 81.0 - 5.0 / 243.0 * t571 * t8228 * t6147 * t3854 * t1861 + 2.0 / 27.0 * t571 * t3031 * t1867 * t10273 * t544 - 2.0 / 81.0 * t29345 + 2.0 / 243.0 * t29347 - t571 * t3031 * t10338 * t1877 / 9.0 + 2.0 / 27.0 * t29353 - 44.0 / 81.0 * t29355 + 2.0 / 27.0 * t571 * t3025 * t10327 * t1877 - 4.0 / 81.0 * t29361;
    (t29363,)
}
