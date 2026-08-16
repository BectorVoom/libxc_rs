//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 814/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk814(t4883: f64, t4898: f64, t1048: f64, t2850: f64, t2867: f64, t3142: f64, t468: f64, t2: f64, t3034: f64, t464: f64, t4968: f64, t4976: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8545 = 20.0_f64 * t4883;
    let t8547 = 12.0_f64 * t4898;
    let t8549 = t1048 * t2867 * t2850;
    let t8550 = 2.0_f64 * t8549;
    let t8551 = t3142 * t468;
    let t8552 = 0.5848223622634646207e0_f64 * t8551;
    let t8553 = t3034 * t2;
    let t8554 = t8553 * t464;
    let t8555 = 0.18311447306006545054e-3_f64 * t8554;
    let t8556 = 0.10843581300301739842e-1_f64 * t4968;
    let t8559 = 32.0_f64 * t4976;
    (t8545, t8547, t8550, t8552, t8555, t8556, t8559)
}
