//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 965/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk965<F: Float>(t1114: F, t19817: F, t19905: F, t19839: F, t833: F, t1146: F, t6729: F, t1125: F, t21121: F, t20189: F, t3133: F, t20693: F) -> (F, F, F, F, F, F, F) {
    let t26755 = t1114 * t19817;
    let t26958 = t1114 * t19905;
    let t27077 = t1114 * t19839 * t833;
    let t27079 = t6729 * t1146;
    let t27197 = t1125 * t21121;
    let t27222 = t20189 * t3133;
    let t27556 = t1114 * t20693;
    (t26755, t26958, t27077, t27079, t27197, t27222, t27556)
}
