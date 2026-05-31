//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 925/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk925<F: Float>(t10365: F, t610: F, t1885: F, t1820: F, t1648: F, t3527: F, t591: F, t9788: F, t590: F, t587: F, t3531: F, t1802: F, t3454: F) -> (F, F, F, F, F) {
    let t10366 = t10365 * t610;
    let t10367 = t1885 * t10366;
    let t10369 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1820 * t10367;
    let t10371 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t1648 * t3527;
    let t10372 = t591 * t9788;
    let t10373 = t590 * t10372;
    let t10375 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t587 * t10373;
    let t10377 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1648 * t3531;
    let t10378 = t1802 * t3454;
    (t10369, t10371, t10375, t10377, t10378)
}
