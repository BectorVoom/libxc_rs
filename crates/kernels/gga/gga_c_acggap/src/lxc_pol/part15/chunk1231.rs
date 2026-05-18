//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1231/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1231<F: Float>(t34879: F, t34897: F, t34945: F, t34961: F, t34986: F, t34987: F, t37282: F, t37291: F, t37292: F, t37311: F, t39551: F, t39555: F, t39557: F, t39559: F, t39563: F, t39567: F, t39570: F, t39574: F) -> F {
    let t41706 = -F::new(0.21437009059034868486e-3) * t39551 - F::new(0.21437009059034868486e-3) * t39555 - t37282 + F::new(0.17149607247227894789e-2) * t34879 + t37291 + t37292 - F::new(0.26147916666666666667e0) * t34897 - t39557 / F::new(12.0) - t39559 / F::new(12.0) - F::new(0.18868855373762491241e-1) * t39563 - F::new(0.37737710747524982484e-2) * t34945 + t37311 - F::new(0.6289618457920830414e-2) * t34961 - t34986 - t34987 + F::new(0.21437009059034868486e-2) * t39567 + F::new(0.21437009059034868486e-2) * t39570 + F::new(0.21437009059034868486e-2) * t39574;
    t41706
}
