//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1253/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1253<F: Float>(t1267: F, t26996: F, t5329: F, t6837: F, t1020: F, t19557: F, t7718: F, t19561: F, t4994: F, t6620: F, t92917: F, t100129: F, t27077: F) -> (F, F, F, F, F) {
    let t100466 = t5329 * t26996 * t6837 * t1267;
    let t100474 = t1020 * t7718 * t19557;
    let t100477 = t4994 * t7718 * t19561;
    let t100480 = t1020 * t92917 * t6620;
    let t100482 = t27077 * t100129;
    (t100466, t100474, t100477, t100480, t100482)
}
