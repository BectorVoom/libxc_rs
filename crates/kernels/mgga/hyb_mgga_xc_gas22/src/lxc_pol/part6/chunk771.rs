//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 771/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk771<F: Float>(t2521: F, t4273: F, t2527: F, t3461: F, t4236: F, t1421: F, t987: F) -> (F, F, F, F) {
    let t4275 = 0.16081979498692535067e2 * t2521 * t4273;
    let t4278 = t2527 - 0.34246666666666666666e-1 * t3461 + 0.5137e-1 * t4236;
    let t4283 = t1421 * t1421;
    let t4284 = t4283 * t987;
    (t4275, t4278, t4283, t4284)
}
