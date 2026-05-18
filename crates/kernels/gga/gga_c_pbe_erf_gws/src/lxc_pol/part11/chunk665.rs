//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 665/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk665<F: Float>(t1764: F, t5219: F, t1660: F, t597: F, t1663: F, t2650: F, t723: F, t995: F, t1022: F, t5212: F, t108: F, t210: F) -> (F, F, F, F, F, F, F) {
    let t7063 = t5219 * t1764;
    let t7068 = t1660 * t597;
    let t7069 = t7068 * t1663;
    let t7075 = t2650 * t723;
    let t7087 = t995 * t1764;
    let t7106 = t5212 * t1022;
    let t7114 = t210 * t108;
    (t7063, t7068, t7069, t7075, t7087, t7106, t7114)
}
