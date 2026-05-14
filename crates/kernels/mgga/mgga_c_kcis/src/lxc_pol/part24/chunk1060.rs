//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1060/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1060<F: Float>(t26420: F, t27731: F, t27733: F, t27735: F, t27737: F, t27739: F, t27744: F, t27747: F, t27750: F, t27753: F, t27756: F, t1141: F, t27985: F, t283: F, t5164: F, t5082: F, t982: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t93852 = 12.0 * t26420;
    let t95270 = t27731 / 8.0;
    let t95271 = 2.0 * t27733;
    let t95272 = t27735 / 8.0;
    let t95273 = t27737 / 8.0;
    let t95274 = t27739 / 8.0;
    let t95276 = t27744 / 8.0;
    let t95278 = t27747 / 8.0;
    let t95279 = t27750 / 8.0;
    let t95280 = t27753 / 8.0;
    let t95281 = t27756 / 8.0;
    let t95286 = t27985 * t1141;
    let t95321 = t5164 * t283;
    let t95326 = t5082 * t982;
    (t93852, t95270, t95271, t95272, t95273, t95274, t95276, t95278, t95279, t95280, t95281, t95286, t95321, t95326)
}
