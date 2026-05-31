//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1058/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1058<F: Float>(t3123: F, t9028: F, t3128: F, t8869: F, t3111: F, t3786: F, t850: F, t860: F, t2848: F, t339: F, t1123: F, t11651: F, t4386: F) -> (F, F, F, F, F, F, F) {
    let t11874 = t3123 * t9028 / F::cast_from(48.0_f64);
    let t11876 = t3128 * t8869 / F::cast_from(8.0_f64);
    let t11878 = t850 * t3111 * t3786;
    let t11880 = t11878 * t860 / F::cast_from(96.0_f64);
    let t11881 = t2848 * t339;
    let t11883 = t850 * t1123 * t11881;
    let t11885 = t11883 * t860 / F::cast_from(96.0_f64);
    let t11886 = t4386 * t11651;
    (t11874, t11876, t11878, t11880, t11883, t11885, t11886)
}
