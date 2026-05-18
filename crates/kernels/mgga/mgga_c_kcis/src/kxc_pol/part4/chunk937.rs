//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 937/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk937<F: Float>(t2605: F, t823: F, t2489: F, t804: F, t2594: F, t158: F, t2490: F, t160: F, t774: F, t2526: F, t2612: F, t8531: F) -> (F, F, F, F) {
    let t9040 = t2605 * t823;
    let t9042 = t804 * t2489;
    let t9043 = t9042 * t2594;
    let t9045 = t2490 * t158;
    let t9046 = t160 * t774;
    let t9047 = t9046 * t2526;
    let t9048 = t9045 * t9047;
    let t9050 = t8531 * t2612;
    (t9040, t9043, t9048, t9050)
}
