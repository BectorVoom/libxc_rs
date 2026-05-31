//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 609/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk609<F: Float>(t75: F, t959: F, t472: F, t414: F, t960: F, t409: F, t1267: F, t1271: F, t1394: F, t1398: F, t1446: F, t2510: F, t2511: F, t2514: F, t2516: F, t2517: F) -> (F, F, F, F, F) {
    let t2840 = t959 * t75;
    let t2841 = t2840 * t472;
    let t2842 = F::cast_from(0.58482233974552040708e0_f64) * t2841;
    let t2843 = t414 * t960;
    let t2844 = F::cast_from(4.0_f64) * t2843;
    let t2845 = t409 * t960;
    let t2846 = F::cast_from(4.0_f64) * t2845;
    let t2847 = -t2510 - t1271 - t2511 + t1446 + t2514 + t2516 - t1267 - t1394 - t1398 - t2517 - t2842 - t2844 + t2846;
    (t2840, t2842, t2844, t2846, t2847)
}
