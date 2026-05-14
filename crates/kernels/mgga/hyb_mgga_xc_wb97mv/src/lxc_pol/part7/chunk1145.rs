//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1145/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1145<F: Float>(t1045: F, t7500: F, t22469: F, t2640: F, t2639: F, t16: F, t8223: F, t1022: F, t15: F, t221: F, t435: F, t12: F, t222: F, t23604: F, t7513: F, t2651: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23608 = t7500 * t1045;
    let t23610 = t2640 * t22469;
    let t23611 = t2639 * t23610;
    let t23613 = t16 * t8223;
    let t23614 = t1022 * t23613;
    let t23616 = t15 * t8223;
    let t23617 = t221 * t23616;
    let t23619 = f64::powf(t435, -0.25e1);
    let t23622 = t23619 * t12 * t23604 * t222;
    let t23624 = t7513 * t1045;
    let t23626 = t2651 * t23610;
    (t23608, t23611, t23613, t23614, t23616, t23617, t23622, t23624, t23626)
}
