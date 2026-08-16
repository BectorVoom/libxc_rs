//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 813/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk813<F: Float>(t2096: F, t2454: F, t4498: F, t19: F, t3025: F, t796: F, t801: F, t1105: F, t945: F, t810: F, t2474: F, t460: F) -> (F, F, F, F, F) {
    let t6906 = t2454 * t2096;
    let t6918 = F::cast_from(4.0_f64) * t4498;
    let t6921 = t3025 * t796 * t19;
    let t6922 = t6921 * t801;
    let t6923 = F::cast_from(0.82152657680133333336e0_f64) * t6922;
    let t6925 = t945 * t1105;
    let t6926 = t6925 * t810;
    let t6930 = t2474 * t460;
    (t6906, t6918, t6923, t6926, t6930)
}
