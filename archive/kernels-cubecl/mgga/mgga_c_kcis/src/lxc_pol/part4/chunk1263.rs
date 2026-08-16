//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1263/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1263<F: Float>(t16106: F, t16107: F, t3944: F, t5613: F, t5619: F, t1315: F, t5538: F, t1336: F, t3894: F, t5541: F, t1893: F, t3898: F) -> (F, F, F, F, F) {
    let t16108 = t16106 * t16107;
    let t16111 = t3944 * t5613;
    let t16112 = t16111 * t5619;
    let t16115 = t5538 * t1315;
    let t16117 = F::cast_from(2.0_f64) * t16115 * t1336;
    let t16119 = F::cast_from(1.0_f64) * t5541 * t3894;
    let t16120 = t1893 * t3898;
    (t16108, t16112, t16117, t16119, t16120)
}
