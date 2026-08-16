//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1249/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1249(t25064: f64, t81902: f64, t7521: f64, t81632: f64, t22690: f64, t23171: f64, t25319: f64, t23143: f64, t7525: f64, t25316: f64, t82038: f64, t23228: f64, t7488: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87445 = t81902 * t25064;
    let t87635 = t81632 * t7521;
    let t87653 = t23171 * t22690 * t25319;
    let t87666 = t23143 * t7525;
    let t87718 = t82038 * t25316;
    let t87779 = t23171 * t23228 * t7488;
    (t87445, t87635, t87653, t87666, t87718, t87779)
}
