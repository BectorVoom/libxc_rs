//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1117/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1117<F: Float>(t2373: F, t4424: F, t6127: F, t9296: F, t829: F, t830: F, t4379: F, t831: F, t2370: F, t4417: F, t814: F, t2379: F, t4474: F) -> (F, F, F, F, F) {
    let t20049 = t4424 * t2373;
    let t20051 = t9296 * t6127;
    let t20053 = t829 * t830 * t20051;
    let t20056 = t831 * t4379;
    let t20058 = t2370 * t830 * t20056;
    let t20063 = t829 * t830 * t4417 * t814;
    let t20076 = t4474 * t2379;
    (t20049, t20053, t20058, t20063, t20076)
}
