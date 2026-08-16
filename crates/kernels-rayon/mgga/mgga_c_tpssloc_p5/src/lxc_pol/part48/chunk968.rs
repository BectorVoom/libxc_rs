//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 968/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk968(t23204: f64, t31419: f64, t6562: f64, t112946: f64, t112949: f64, t113038: f64, t113041: f64, t113045: f64, t114937: f64, t114939: f64, t114944: f64, t114945: f64, t114960: f64, t22974: f64, t23191: f64, t24325: f64, t25168: f64, t259: f64, t26728: f64, t2718: f64, t2720: f64, t2742: f64, t31361: f64, t31423: f64, t6627: f64, t7087: f64, t798: f64, t855: f64, t8562: f64) -> f64 {
    let t114965 = t6562 * t23204 * t31419;
    let t114967 = -0.82246703342411321825e-2_f64 * t114937 + 0.38381794893125283518e-1_f64 * t114939 - t7087 * t23191 + t114944 + t112946 + t112949 + 0.38381794893125283518e-1_f64 * t114945 + t113038 + 2.0_f64 * t798 * t31361 * t259 + t113041 - t113045 + 2.0_f64 * t855 * t2718 * t8562 * t2742 - 6.0_f64 * t25168 * t26728 * t22974 + 4.0_f64 * t6627 * t24325 - 0.16449340668482264365e-1_f64 * t114960 + 2.0_f64 * t31423 * t2720 + 0.82246703342411321824e-2_f64 * t114965;
    t114967
}
