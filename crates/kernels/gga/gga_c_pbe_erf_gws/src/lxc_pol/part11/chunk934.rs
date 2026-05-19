//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 934/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk934<F: Float>(t20303: F, t2252: F, t6241: F, t19803: F, t346: F, t2306: F, t6643: F, t6472: F, t6670: F, t2250: F, t6201: F, t933: F, param_a_c: F) -> (F, F, F, F, F, F, F) {
    let t20304 = t20303 * t2252;
    let t20307 = t6241 * param_a_c;
    let t20378 = t19803 * t346;
    let t20411 = t2306 * t6643;
    let t20432 = t6472 * t346;
    let t20490 = t2306 * t6670;
    let t20521 = t2250 * t6201 * t933;
    (t20304, t20307, t20378, t20411, t20432, t20490, t20521)
}
