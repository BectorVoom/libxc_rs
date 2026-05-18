//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 729/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk729<F: Float>(t1105: F, t1134: F, t858: F, t2407: F, t1114: F, t8987: F, t3123: F, t8824: F, t3854: F, t5: F, t6: F, t3824: F) -> (F, F, F, F, F, F, F, F) {
    let t11412 = t1134 * t1105;
    let t11413 = t858 * t11412;
    let t11414 = t2407 * t11413;
    let t11419 = t1114 * t8987;
    let t11447 = t3123 * t8824;
    let t11459 = t5 * t3854;
    let t11464 = t6 * t3854;
    let t11478 = t5 * t3824;
    (t11412, t11413, t11414, t11419, t11447, t11459, t11464, t11478)
}
