//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1164/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1164<F: Float>(t1021: F, t19589: F, t1092: F, t1121: F, t6496: F, t3218: F, t1096: F, t3203: F, t6276: F, t3202: F, t3200: F, t2822: F, t6505: F) -> (F, F, F, F, F) {
    let t19590 = t1021 * t19589;
    let t19591 = t1092 * t19590;
    let t19593 = t6496 * t1121;
    let t19594 = t3218 * t19593;
    let t19595 = t1096 * t19594;
    let t19596 = t1092 * t19595;
    let t19599 = t3203 * t6276 * t1121;
    let t19600 = t3202 * t19599;
    let t19601 = t3200 * t19600;
    let t19603 = t2822 * t6505;
    (t19591, t19593, t19596, t19601, t19603)
}
