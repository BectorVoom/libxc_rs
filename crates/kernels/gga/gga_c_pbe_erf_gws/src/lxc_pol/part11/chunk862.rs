//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 862/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk862<F: Float>(t1041: F, t2718: F, t1028: F, t2704: F, t1022: F, t5109: F, t108: F, t267: F, t2789: F, t1917: F, t2519: F, t1062: F, t5385: F, t1045: F, t2735: F, t211: F) -> (F, F, F, F, F, F, F) {
    let t24980 = t2718 * t1041;
    let t25049 = t2704 * t1028;
    let t25081 = t5109 * t1022;
    let t25208 = t2789 * t108 * t267;
    let t25230 = t2519 * t1917;
    let t25349 = t1062 * t5385;
    let t25353 = t2735 * t1045;
    let t25354 = t211 * t25353;
    (t24980, t25049, t25081, t25208, t25230, t25349, t25354)
}
