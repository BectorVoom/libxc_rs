//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1165/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1165<F: Float>(t2855: F, t6334: F, t1021: F, t1020: F, t1121: F, t6486: F, t1022: F, t9589: F, t1092: F, t1133: F, t1131: F, t3227: F) -> (F, F, F, F, F) {
    let t19605 = t2855 * t6334;
    let t19606 = t1021 * t19605;
    let t19607 = t1020 * t19606;
    let t19609 = t6486 * t1121;
    let t19610 = t1022 * t19609;
    let t19611 = t9589 * t19610;
    let t19612 = t1092 * t19611;
    let t19614 = t6486 * t1133;
    let t19615 = t1131 * t19614;
    let t19616 = t3227 * t19615;
    (t19607, t19609, t19612, t19614, t19616)
}
