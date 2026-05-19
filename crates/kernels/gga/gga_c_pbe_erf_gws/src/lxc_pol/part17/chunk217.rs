//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 217/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk217<F: Float>(t422: F, t626: F, t625: F, t11: F, t624: F, t203: F, t184: F) -> (F, F, F, F, F, F) {
    let t627 = t626 * t422;
    let t628 = t625 * t627;
    let t629 = t11 * t628;
    let t631 = t624 + F::cast_from(0.18891666666666666667e-2_f64) * t629;
    let t632 = t203 * t631;
    let t633 = t632 * t184;
    (t627, t628, t629, t631, t632, t633)
}
