//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 970/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk970<F: Float>(t401: F, t5039: F, t5030: F, t190: F, t212: F, t367: F, t16991: F, t17001: F, t17009: F, t17011: F, t17014: F, t17016: F, t17024: F, t17030: F, t17032: F, t1714: F, t25: F, t5061: F, t657: F) -> F {
    let t17968 = t401 * t5039;
    let t17979 = t401 * t5030;
    let t17983 = F::cast_from(0.10864197530864197531e0_f64) * t190 * t367 * t212;
    let t17989 = F::cast_from(0.53333333333333333332e-1_f64) * t25 * t657 * t16991 - F::cast_from(0.10666666666666666667e0_f64) * t17968 + F::cast_from(0.79999999999999999998e-1_f64) * t25 * t1714 * t17001 - F::cast_from(0.88888888888888888888e-2_f64) * t25 * t1714 * t17009 - F::cast_from(0.17777777777777777778e-1_f64) * t25 * t5061 * t17014 + F::cast_from(0.17777777777777777778e-1_f64) * t17979 + t17983 - F::cast_from(0.9597777777777777778e-1_f64) * t17011 - F::cast_from(0.23994444444444444446e0_f64) * t17016 - F::new(0.12957e1) * t17024 - F::cast_from(0.28793333333333333333e0_f64) * t17030 + F::cast_from(0.95977777777777777777e-1_f64) * t17032;
    t17989
}
