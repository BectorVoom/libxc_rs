//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1317/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1317<F: Float>(t11689: F, t14007: F, t14535: F, t3108: F, t11953: F, t14015: F, t11803: F, t3065: F, t36897: F, t858: F, t9119: F, t11648: F, t14101: F) -> (F, F, F, F, F, F) {
    let t57075 = t14007 * t11689;
    let t57077 = t3108 * t14535;
    let t57079 = t14015 * t11953;
    let t57082 = t14007 * t11803;
    let t57086 = t9119 * t3065 * t858 * t36897;
    let t57088 = t14101 * t11648;
    (t57075, t57077, t57079, t57082, t57086, t57088)
}
