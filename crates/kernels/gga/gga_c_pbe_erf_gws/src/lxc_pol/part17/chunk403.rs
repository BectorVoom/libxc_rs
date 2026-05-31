//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 403/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk403<F: Float>(t1396: F, t470: F, t427: F, t75: F, t472: F, t92: F, t418: F) -> (F, F, F, F, F, F) {
    let t1397 = t470 * t1396;
    let t1398 = F::cast_from(0.58482233974552040708e0_f64) * t1397;
    let t1399 = t427 * t75;
    let t1400 = t1399 * t472;
    let t1401 = F::cast_from(0.11696446794910408142e1_f64) * t1400;
    let t1402 = F::cast_from(1.0_f64) / t92;
    let t1403 = t418 * t418;
    (t1398, t1399, t1400, t1401, t1402, t1403)
}
