//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 760/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk760<F: Float>(t2096: F, t2454: F, t4498: F, t19: F, t3025: F, t796: F, t801: F, t1105: F, t945: F, t810: F, t2474: F, t460: F, t40: F, t4: F, t959: F, t1448: F) -> (F, F, F, F, F, F) {
    let t6906 = t2454 * t2096;
    let t6918 = 4.0 * t4498;
    let t6921 = t3025 * t796 * t19;
    let t6922 = t6921 * t801;
    let t6923 = 0.82152657680133333336e0 * t6922;
    let t6925 = t945 * t1105;
    let t6926 = t6925 * t810;
    let t6930 = t2474 * t460;
    let t6931 = t40 * t6930;
    let t6932 = 2.0 * t6931;
    let t6967 = t959 * t4;
    let t6968 = t6967 * t1448;
    (t6906, t6918, t6923, t6926, t6932, t6968)
}
