//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 869/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk869<F: Float>(t2508: F, t2580: F, t28023: F, t2958: F, t3009: F, t7226: F, t23575: F, t3459: F, t11135: F, t7324: F, t10802: F, t23555: F, t42944: F, t5241: F, t5640: F, t590: F) -> (F, F, F, F, F, F) {
    let t43335 = 0.92286314761706691403e-1 * t2508 * t2580 * t2958 * t28023;
    let t43339 = 0.46143157380853345701e-1 * t2508 * t7226 * t3009 * t28023;
    let t43346 = 4.0 * t23575 * t3459;
    let t43353 = 4.0 * t7324 * t11135;
    let t43355 = 12.0 * t23555 * t10802;
    let t43361 = 0.13803453343411469884e2 * t5640 * t5241 * t42944 * t590;
    (t43335, t43339, t43346, t43353, t43355, t43361)
}
