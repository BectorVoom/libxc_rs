//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1363/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1363<F: Float>(t2804: F, t34519: F, t4419: F, t34468: F, t9725: F, t10004: F, t33176: F, t116380: F, t116354: F, t116357: F, t116361: F, t116375: F, t116378: F, t116388: F, t116391: F, t33180: F, t33284: F, t34474: F, t9728: F, t9995: F) -> (F, F) {
    let t117897 = 0.34722222222222222222e-2 * t2804 * t4419 * t34519;
    let t117898 = t4419 * t34468;
    let t117900 = 0.13402777777777777778e-2 * t9725 * t117898;
    let t117903 = t33176 * t10004;
    let t117906 = 0.15476481481481481481e-2 * t116380;
    let t117911 = 0.19345601851851851852e-2 * t116354 + 0.51588271604938271605e-2 * t116357 + 0.77382407407407407406e-3 * t116361 + 0.52083333333333333333e-2 * t33284 * t9995 + t117897 + t117900 - 0.23214722222222222222e-2 * t116375 - 0.11607361111111111111e-1 * t116378 + 0.31040833333333333334e-2 * t117903 * t33180 - t117906 + 0.61905925925925925924e-2 * t116388 - 0.41270617283950617282e-2 * t116391 + 0.10416666666666666667e-1 * t34474 * t9728;
    (t117898, t117911)
}
