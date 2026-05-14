//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1110/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1110<F: Float>(t26933: F, t5062: F, t14778: F, t7748: F, t1796: F, t3362: F, t15056: F, t377: F, t283: F, t5164: F, t7755: F, t1200: F, t13181: F, t5082: F, t982: F, t7749: F) -> (F, F, F, F, F, F, F) {
    let t95313 = t26933 * t5062;
    let t95315 = t7748 * t14778;
    let t95317 = t1796 * t3362;
    let t95319 = t15056 * t377;
    let t95321 = t5164 * t283;
    let t95322 = t95321 * t7755;
    let t95324 = t13181 * t1200;
    let t95326 = t5082 * t982;
    let t95327 = t95326 * t7749;
    (t95313, t95315, t95317, t95319, t95322, t95324, t95327)
}
