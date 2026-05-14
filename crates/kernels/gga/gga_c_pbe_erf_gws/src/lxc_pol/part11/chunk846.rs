//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 846/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk846<F: Float>(t4422: F, t828: F, t2331: F, t362: F, t915: F, t2250: F, t4395: F, t6670: F, t356: F, t358: F, t6552: F, t2252: F, t6241: F, t19803: F, t346: F, t2306: F, t6643: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20189 = t4422 * t828;
    let t20269 = t362 * t2331;
    let t20270 = t20269 * t915;
    let t20271 = t2250 * t20270;
    let t20281 = t4395 * t6670;
    let t20303 = t356 * t358 * t6552;
    let t20304 = t20303 * t2252;
    let t20307 = t6241 * param_a_c;
    let t20378 = t19803 * t346;
    let t20411 = t2306 * t6643;
    (t20189, t20269, t20270, t20271, t20281, t20304, t20307, t20378, t20411)
}
