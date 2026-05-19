//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 814/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk814<F: Float>(t4883: F, t4898: F, t1048: F, t2850: F, t2867: F, t3142: F, t468: F, t2: F, t3034: F, t464: F, t4968: F, t4976: F) -> (F, F, F, F, F, F, F) {
    let t8545 = F::new(20.0) * t4883;
    let t8547 = F::new(12.0) * t4898;
    let t8549 = t1048 * t2867 * t2850;
    let t8550 = F::new(2.0) * t8549;
    let t8551 = t3142 * t468;
    let t8552 = F::cast_from(0.5848223622634646207e0_f64) * t8551;
    let t8553 = t3034 * t2;
    let t8554 = t8553 * t464;
    let t8555 = F::cast_from(0.18311447306006545054e-3_f64) * t8554;
    let t8556 = F::cast_from(0.10843581300301739842e-1_f64) * t4968;
    let t8559 = F::new(32.0) * t4976;
    (t8545, t8547, t8550, t8552, t8555, t8556, t8559)
}
