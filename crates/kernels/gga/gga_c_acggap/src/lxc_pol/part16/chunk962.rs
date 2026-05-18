//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 962/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk962<F: Float>(t4680: F, t7346: F, t8896: F, t7433: F, t8962: F, t30374: F, t8657: F, t30811: F, t4904: F, t2450: F, t7431: F, t8461: F) -> (F, F, F, F, F) {
    let t34130 = t7346 * t4680 * t8896;
    let t34131 = F::new(0.21437009059034868486e-3) * t34130;
    let t34132 = t7433 * t8962;
    let t34133 = F::new(0.37737710747524982482e-2) * t34132;
    let t34156 = t30374 * t8657;
    let t34158 = t30811 * t4904;
    let t34159 = F::new(0.68598428988911579156e-2) * t34158;
    let t34161 = t2450 * t7431 * t8461;
    (t34131, t34133, t34156, t34159, t34161)
}
