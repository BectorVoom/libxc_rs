//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 756/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk756<F: Float>(t4883: F, t4898: F, t1048: F, t2850: F, t2867: F, t3142: F, t468: F, t2: F, t3034: F, t464: F, t4968: F, t4976: F, t6959: F, t2999: F, t4938: F, t1361: F, t3002: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8545 = 20.0 * t4883;
    let t8547 = 12.0 * t4898;
    let t8549 = t1048 * t2867 * t2850;
    let t8550 = 2.0 * t8549;
    let t8551 = t3142 * t468;
    let t8552 = 0.5848223622634646207e0 * t8551;
    let t8553 = t3034 * t2;
    let t8554 = t8553 * t464;
    let t8555 = 0.18311447306006545054e-3 * t8554;
    let t8556 = 0.10843581300301739842e-1 * t4968;
    let t8559 = 32.0 * t4976;
    let t8560 = 0.21687162600603479684e-1 * t6959;
    let t8561 = t4938 * t2999;
    let t8566 = t1361 * t3002;
    (t8545, t8547, t8550, t8552, t8555, t8556, t8559, t8560, t8561, t8566)
}
