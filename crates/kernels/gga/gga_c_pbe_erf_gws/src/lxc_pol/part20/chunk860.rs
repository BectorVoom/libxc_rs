//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 860/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk860<F: Float>(t3342: F, t476: F, t3346: F, t92: F, t3351: F, t478: F, t3354: F, t93: F, t108: F, t1351: F, t2538: F, t2544: F, t418: F, t422: F, t726: F, t728: F, t9788: F, t9801: F) -> (F,) {
    let t10636 = t476 * t3342;
    let t10641 = t92 * t3346;
    let t10646 = t478 * t3351;
    let t10651 = t93 * t3354;
    let t10657 = (40.0 / 27.0 * t10636 * t418 + 80.0 / 9.0 * t2538 * t1351 + 20.0 / 9.0 * t10641 * t418 + 4.0 / 3.0 * t726 * t9788 + 40.0 / 27.0 * t10646 * t422 - 80.0 / 9.0 * t2544 * t1351 + 20.0 / 9.0 * t10651 * t422 + 4.0 / 3.0 * t728 * t9801) * t108;
    (t10657,)
}
