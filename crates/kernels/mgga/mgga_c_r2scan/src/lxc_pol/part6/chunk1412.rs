//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1412/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1412<F: Float>(t1732: F, t7829: F, t22166: F, t7693: F, t22358: F, t22362: F, t22365: F, t26700: F, t26702: F, t26704: F, t26706: F, t26708: F, t26710: F, t26712: F, t26715: F, t26718: F) -> (F,) {
    let t26720 = t7829 * t1732;
    let t26721 = 0.300153217574e-2 * t26720;
    let t26724 = t7693 * t22166;
    let t26725 = 0.12154685976e1 * t26724;
    let t26726 = -0.571528e-1 * t22358 - t26700 - 0.1714584e0 * t26702 + 0.28895839882605942647e1 * t26704 - 0.97592231702715658578e-1 * t26706 + 0.80040858019733333332e-2 * t26708 + 0.80040858019733333332e-2 * t26710 + 0.40020429009866666664e-2 * t26712 - 0.60030643514799999999e-2 * t26715 - 0.1200612870296e-1 * t26718 - t26721 - 60.0 * t22362 + 0.15431256e1 * t22365 - t26725;
    (t26726,)
}
