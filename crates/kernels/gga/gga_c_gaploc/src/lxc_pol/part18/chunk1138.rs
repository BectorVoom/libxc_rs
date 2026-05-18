//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1138/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1138<F: Float>(t28229: F, t3192: F, t574: F, t1641: F, t9421: F, t18364: F, t6710: F, t9438: F, t7014: F, t9552: F, t20843: F, t2487: F, t3177: F) -> (F, F, F, F, F) {
    let t30542 = t574 * t28229 * t3192;
    let t30546 = t1641 * t9421;
    let t30572 = t6710 * t9438 * t18364;
    let t30574 = t7014 * t9552;
    let t30575 = F::new(0.1022478025437886658e1) * t30574;
    let t30578 = F::new(0.11928910296775344344e1) * t2487 * t20843 * t3177;
    (t30542, t30546, t30572, t30575, t30578)
}
