//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1321/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1321<F: Float>(t11896: F, t4049: F, t11475: F, t4028: F, t11734: F, t4043: F, t15255: F, t51382: F, t3799: F, t4033: F, t3867: F, t11573: F, t14015: F) -> (F, F, F, F, F, F, F) {
    let t57134 = t4049 * t11896;
    let t57138 = t4028 * t11475;
    let t57140 = t4043 * t11734;
    let t57142 = t51382 * t15255;
    let t57144 = t4033 * t3799;
    let t57146 = t4033 * t3867;
    let t57151 = t14015 * t11573;
    (t57134, t57138, t57140, t57142, t57144, t57146, t57151)
}
