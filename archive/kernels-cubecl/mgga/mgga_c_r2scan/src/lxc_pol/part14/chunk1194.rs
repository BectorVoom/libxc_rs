//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1194/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1194<F: Float>(t11342: F, t40664: F, t11199: F, t11497: F, t3262: F, t11338: F, t40276: F, t3579: F, t495: F, t797: F, t11189: F, t3275: F, t39209: F) -> (F, F, F, F, F) {
    let t41240 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t40664 * t11342;
    let t41243 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t3262 * t11199 * t11497;
    let t41247 = t40276 * t11338 / F::cast_from(2.0_f64);
    let t41251 = t3579 * t495 * t11199 * t797 / F::cast_from(2.0_f64);
    let t41254 = F::cast_from(45.0_f64) / F::cast_from(32.0_f64) * t3275 * t11189 * t39209;
    (t41240, t41243, t41247, t41251, t41254)
}
