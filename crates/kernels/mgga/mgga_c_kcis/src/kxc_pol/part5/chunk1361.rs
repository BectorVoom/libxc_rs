//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1361/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1361<F: Float>(t22403: F, t4260: F, t21905: F, t5909: F, t21038: F, t5908: F, t12530: F, t7299: F, t12575: F, t7318: F, t12568: F, t7338: F) -> (F, F, F, F, F, F) {
    let t22404 = t4260 * t22403;
    let t22406 = t5909 * t21905;
    let t22407 = t4260 * t22406;
    let t22410 = t5909 * t21038;
    let t22411 = t5908 * t22410;
    let t22413 = t12530 * t7299;
    let t22415 = t12575 * t7318;
    let t22417 = t12568 * t7338;
    (t22404, t22407, t22411, t22413, t22415, t22417)
}
