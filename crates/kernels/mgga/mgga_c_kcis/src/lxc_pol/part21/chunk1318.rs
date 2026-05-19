//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1318/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1318<F: Float>(t26714: F, t8030: F, t1009: F, t14395: F, t13330: F, t3200: F, t92808: F, t2178: F, t3217: F, t14068: F, t1020: F, t13292: F, t7718: F) -> (F, F, F, F, F) {
    let t96238 = F::cast_from(0.46336805555555555556e-3_f64) * t8030 * t26714;
    let t96241 = t14395 * t1009;
    let t96247 = t3200 * t92808 * t13330;
    let t96249 = t2178 * t3217;
    let t96251 = t3200 * t96249 * t14068;
    let t96256 = t1020 * t7718 * t13292;
    (t96238, t96241, t96247, t96251, t96256)
}
