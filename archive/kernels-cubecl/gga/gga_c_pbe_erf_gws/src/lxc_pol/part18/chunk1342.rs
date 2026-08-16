//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1342/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1342<F: Float>(t14547: F, t20842: F, t38545: F, t37454: F, t6523: F, t11461: F, t4028: F, t11896: F, t4049: F, t11475: F, t11734: F, t4043: F) -> (F, F, F, F, F, F) {
    let t57127 = t14547 * t20842 * t38545;
    let t57130 = t14547 * t6523 * t37454;
    let t57132 = t4028 * t11461;
    let t57134 = t4049 * t11896;
    let t57138 = t4028 * t11475;
    let t57140 = t4043 * t11734;
    (t57127, t57130, t57132, t57134, t57138, t57140)
}
