//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1231/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1231(t34879: f64, t34897: f64, t34945: f64, t34961: f64, t34986: f64, t34987: f64, t37282: f64, t37291: f64, t37292: f64, t37311: f64, t39551: f64, t39555: f64, t39557: f64, t39559: f64, t39563: f64, t39567: f64, t39570: f64, t39574: f64) -> f64 {
    let t41706 = -0.21437009059034868486e-3_f64 * t39551 - 0.21437009059034868486e-3_f64 * t39555 - t37282 + 0.17149607247227894789e-2_f64 * t34879 + t37291 + t37292 - 0.26147916666666666667e0_f64 * t34897 - t39557 / 12.0_f64 - t39559 / 12.0_f64 - 0.18868855373762491241e-1_f64 * t39563 - 0.37737710747524982484e-2_f64 * t34945 + t37311 - 0.6289618457920830414e-2_f64 * t34961 - t34986 - t34987 + 0.21437009059034868486e-2_f64 * t39567 + 0.21437009059034868486e-2_f64 * t39570 + 0.21437009059034868486e-2_f64 * t39574;
    t41706
}
