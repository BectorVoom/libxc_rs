//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 941/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk941<F: Float>(t11387: F, t16676: F, t16677: F, t11794: F, t7420: F, t11320: F, t2619: F, t7921: F, t11499: F, t2629: F, t933: F, t772: F, t9786: F, t9787: F, t11948: F, t29350: F) -> (F, F, F, F, F, F) {
    let t33173 = t16676 * t11387 * t16677;
    let t33175 = t11794 * t7420;
    let t33179 = t2619 * t11320 * t7921;
    let t33182 = t933 * t11499 * t2629;
    let t33185 = t9786 * t772 * t9787;
    let t33187 = t11948 * t29350;
    (t33173, t33175, t33179, t33182, t33185, t33187)
}
