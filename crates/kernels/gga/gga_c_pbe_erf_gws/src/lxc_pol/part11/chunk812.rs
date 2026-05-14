//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 812/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk812<F: Float>(t1651: F, t1802: F, t5293: F, t597: F, t5283: F, t1642: F, t212: F, t22: F, t16972: F, t219: F, t5400: F, t649: F, t5399: F, t9: F, t17037: F, t155: F, t188: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17252 = t1651 * t1802;
    let t17260 = t5293 * t597;
    let t17268 = t5283 * t597;
    let t17321 = t22 / t212 / t1642;
    let t17322 = t219 * t16972;
    let t17331 = t5400 * t649;
    let t17440 = t9 * t5399;
    let t17444 = t219 * t17037;
    let t17470 = t155 * t188;
    (t17252, t17260, t17268, t17321, t17322, t17331, t17440, t17444, t17470)
}
