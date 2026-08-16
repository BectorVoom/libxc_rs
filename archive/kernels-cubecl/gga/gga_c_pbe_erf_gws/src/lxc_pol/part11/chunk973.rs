//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 973/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk973<F: Float>(t3499: F, t5463: F, t639: F, t3345: F, t5219: F, t1802: F, t1672: F, t3450: F, t561: F, t3459: F, t678: F, t1791: F, t3562: F) -> (F, F, F, F, F, F) {
    let t31225 = t639 * t5463 * t3499;
    let t31267 = t5219 * t3345;
    let t31352 = t1802 * t3345;
    let t31443 = t561 * t1672 * t3450;
    let t31492 = t3459 * t678;
    let t31503 = t1791 * t3562;
    (t31225, t31267, t31352, t31443, t31492, t31503)
}
