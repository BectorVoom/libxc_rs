//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1156/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1156<F: Float>(t14651: F, t9283: F, t14113: F, t4142: F, t1114: F, t13791: F) -> (F, F, F) {
    let t14652 = t9283 * t14651;
    let t14655 = t14113 * t4142;
    let t14657 = t1114 * t13791;
    (t14652, t14655, t14657)
}
