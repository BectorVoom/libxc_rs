//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1182/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1182<F: Float>(t27750: F, t27753: F, t27756: F, t1141: F, t27985: F, t283: F, t5164: F, t5082: F, t982: F, t14781: F, t1796: F, t26929: F, t5025: F) -> (F, F, F, F, F, F, F, F, F) {
    let t95279 = t27750 / F::new(8.0);
    let t95280 = t27753 / F::new(8.0);
    let t95281 = t27756 / F::new(8.0);
    let t95286 = t27985 * t1141;
    let t95321 = t5164 * t283;
    let t95326 = t5082 * t982;
    let t95351 = t14781 * t283;
    let t95376 = t1796 * t982;
    let t95381 = t5025 * t26929;
    (t95279, t95280, t95281, t95286, t95321, t95326, t95351, t95376, t95381)
}
