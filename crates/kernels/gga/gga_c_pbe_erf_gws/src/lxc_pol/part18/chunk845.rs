//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 845/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk845<F: Float>(t591: F, t9788: F, t590: F, t587: F, t1648: F, t3531: F, t1802: F, t3454: F, t562: F, t1885: F, t1820: F, t5175: F, t610: F, t2630: F, t2784: F, t1017: F, t950: F) -> (F, F, F, F, F, F) {
    let t10372 = t591 * t9788;
    let t10373 = t590 * t10372;
    let t10375 = 4.0 / 45.0 * t587 * t10373;
    let t10377 = 4.0 / 27.0 * t1648 * t3531;
    let t10378 = t1802 * t3454;
    let t10379 = t10378 * t562;
    let t10380 = t1885 * t10379;
    let t10382 = 8.0 / 15.0 * t1820 * t10380;
    let t10383 = t5175 * t3454;
    let t10384 = t10383 * t610;
    let t10385 = t1885 * t10384;
    let t10387 = 4.0 / 5.0 * t587 * t10385;
    let t10388 = t2630 * t2784;
    let t10389 = t1885 * t10388;
    let t10391 = 8.0 / 15.0 * t587 * t10389;
    let t10392 = t950 * t1017;
    (t10375, t10377, t10382, t10387, t10391, t10392)
}
