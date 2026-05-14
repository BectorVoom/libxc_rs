//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 633/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk633<F: Float>(t1448: F, t6967: F, t1069: F, t1617: F, t1022: F, t1791: F, t1660: F, t197: F, t1663: F, t108: F, t182: F, t267: F, t1764: F, t5219: F, t597: F, t2650: F, t723: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6968 = t6967 * t1448;
    let t6998 = t1069 * t1617;
    let t7027 = t1791 * t1022;
    let t7048 = t1660 * t197;
    let t7049 = t7048 * t1663;
    let t7061 = t182 * t108;
    let t7062 = t7061 * t267;
    let t7063 = t5219 * t1764;
    let t7068 = t1660 * t597;
    let t7069 = t7068 * t1663;
    let t7075 = t2650 * t723;
    (t6968, t6998, t7027, t7049, t7062, t7063, t7068, t7069, t7075)
}
