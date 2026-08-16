//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1150/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1150<F: Float>(t1131: F, t14595: F, t1096: F, t1092: F, t3182: F, t4823: F, t4819: F, t9532: F, t4793: F, t9429: F, t2861: F, t4815: F) -> (F, F, F, F, F) {
    let t14596 = t1131 * t14595;
    let t14597 = t1096 * t14596;
    let t14598 = t1092 * t14597;
    let t14600 = t3182 * t4823;
    let t14601 = t1096 * t14600;
    let t14602 = t1092 * t14601;
    let t14604 = t9532 * t4819;
    let t14605 = t1092 * t14604;
    let t14607 = t9429 * t4793;
    let t14609 = t2861 * t4815;
    (t14598, t14602, t14605, t14607, t14609)
}
