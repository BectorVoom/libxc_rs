//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 494/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk494<F: Float>(t4120: F, t4124: F, t1004: F, t589: F, t4165: F, t4167: F, t4169: F, t194: F, t618: F, t1412: F, t171: F, t433: F) -> (F, F, F, F, F, F, F, F) {
    let t5385 = F::new(4.0) * t4120;
    let t5388 = F::new(32.0) * t4124;
    let t5389 = t1004 * t589;
    let t5392 = F::new(0.5848223622634646207e0) * t4165;
    let t5393 = F::new(0.11696447245269292414e1) * t4167;
    let t5394 = F::new(0.34631718211362927518e2) * t4169;
    let t5395 = t194 * t618;
    let t5400 = t1412 * t171;
    let t5402 = F::new(0.11696447245269292414e1) * t5400 * t433;
    (t5385, t5388, t5389, t5392, t5393, t5394, t5395, t5402)
}
