//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 908/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk908<F: Float>(t2365: F, t35611: F, t6111: F, t36762: F, t7785: F, t44712: F, t723: F) -> (F, F, F) {
    let t45414 = t6111 * t2365 * t35611;
    let t45415 = F::new(0.59584149919750711116e-1) * t45414;
    let t45421 = t36762 * t7785;
    let t45423 = t44712 * t723;
    (t45415, t45421, t45423)
}
