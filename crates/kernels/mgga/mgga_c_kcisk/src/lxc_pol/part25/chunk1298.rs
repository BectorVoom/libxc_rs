//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1298/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1298<F: Float>(t17152: F, t1799: F, t33017: F, t15847: F, t15880: F, t9679: F, t112266: F, t112289: F, t112396: F, t116533: F, t116538: F, t116541: F, t116544: F, t116548: F, t116552: F, t32942: F, t33005: F, t33031: F, t33056: F, t34027: F, t34218: F) -> (F, F, F, F) {
    let t116558 = t1799 * t33017 * t17152;
    let t116561 = t1799 * t33017 * t15847;
    let t116564 = t1799 * t9679 * t15880;
    let t116569 = 0.69444444444444444446e-2 * t112266 * t34027 + 0.69444444444444444446e-2 * t112289 * t34027 + 0.69444444444444444446e-2 * t33031 * t116533 + 0.66327777777777777776e-2 * t116538 - 0.55273148148148148146e-2 * t116541 + 0.34722222222222222223e-2 * t33031 * t116544 + 0.46296296296296296297e-2 * t33031 * t116548 - 0.23280625000000000001e-2 * t116552 * t33005 + 0.17870370370370370371e-2 * t33056 * t116548 - 0.33163888888888888888e-2 * t116558 + 0.33163888888888888888e-2 * t116561 + 0.11054629629629629629e-2 * t116564 - 0.69444444444444444446e-2 * t112396 + 0.20833333333333333334e-1 * t32942 * t34218;
    (t116558, t116561, t116564, t116569)
}
