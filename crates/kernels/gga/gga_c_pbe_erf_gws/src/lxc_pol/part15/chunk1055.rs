//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1055/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1055<F: Float>(t3259: F, t814: F, t9441: F, t3257: F, t1113: F, t6110: F, t905: F, t3237: F, t6627: F, t2345: F, t3219: F, t6220: F) -> (F, F, F, F, F, F) {
    let t9568 = t3259 * t814;
    let t9569 = t9441 * t9568;
    let t9570 = t3257 * t9569;
    let t9574 = t1113 * t6110;
    let t9575 = t905 * t9574;
    let t9579 = F::new(7.0) / F::new(1152.0) * t6627 * t3237;
    let t9581 = t2345 * t3219 * t6220;
    (t9569, t9570, t9574, t9575, t9579, t9581)
}
