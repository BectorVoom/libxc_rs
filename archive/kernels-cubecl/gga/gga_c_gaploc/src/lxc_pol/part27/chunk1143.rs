//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1143/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1143<F: Float>(t20358: F, t2365: F, t7025: F, t20731: F, t544: F, t9287: F, t2371: F, t4398: F, t7030: F, t20670: F, t20671: F, t20696: F) -> (F, F, F, F, F) {
    let t30650 = F::cast_from(0.29792074959875355558e-1_f64) * t7025 * t2365 * t20358;
    let t30703 = t544 * t20731;
    let t30705 = F::cast_from(0.59584149919750711116e-1_f64) * t30703 * t9287;
    let t30708 = F::cast_from(0.59584149919750711116e-1_f64) * t4398 * t2371 * t7030;
    let t30712 = F::cast_from(0.17041300423964777634e0_f64) * t20670 * t20671 * t20696;
    (t30650, t30703, t30705, t30708, t30712)
}
