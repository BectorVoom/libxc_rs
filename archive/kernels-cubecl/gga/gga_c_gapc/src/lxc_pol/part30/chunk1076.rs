//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1076/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1076<F: Float>(t772: F, t9786: F, t9787: F, t11948: F, t29350: F, t10039: F, t3438: F, t11479: F, t2767: F, t7294: F, t11748: F, t2594: F) -> (F, F, F, F, F) {
    let t33185 = t9786 * t772 * t9787;
    let t33187 = t11948 * t29350;
    let t33190 = t3438 * t772 * t10039;
    let t33193 = t7294 * t11479 * t2767;
    let t33195 = t11748 * t2594;
    (t33185, t33187, t33190, t33193, t33195)
}
