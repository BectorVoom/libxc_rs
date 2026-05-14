//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 842/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk842<F: Float>(t4888: F, t5312: F, t1820: F, t1821: F, t418: F, t5333: F, t572: F, t1651: F, t1802: F, t5550: F, t587: F, t1829: F, t5304: F, t7136: F, t5293: F, t597: F) -> (F, F, F, F, F, F) {
    let t17246 = 32.0 / 15.0 * t5312 * t4888;
    let t17251 = 32.0 / 45.0 * t1820 * t1821 * t5333 * t572 * t418;
    let t17252 = t1651 * t1802;
    let t17254 = t587 * t17252 * t5550;
    let t17255 = 64.0 / 45.0 * t17254;
    let t17257 = 32.0 / 15.0 * t5304 * t1829;
    let t17259 = 32.0 / 15.0 * t7136 * t4888;
    let t17260 = t5293 * t597;
    (t17246, t17251, t17255, t17257, t17259, t17260)
}
