//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 777/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk777<F: Float>(t2627: F, t5312: F, t1022: F, t1791: F, t1793: F, t1621: F, t1620: F, t1893: F, t2612: F, t1044: F, t5109: F, t639: F, t641: F, t837: F, t2592: F, t2597: F, t5493: F) -> (F, F, F, F, F, F) {
    let t7026 = 8.0 / 15.0 * t5312 * t2627;
    let t7027 = t1791 * t1022;
    let t7028 = t7027 * t1793;
    let t7029 = t1621 * t7028;
    let t7031 = 8.0 / 15.0 * t1620 * t7029;
    let t7033 = 8.0 / 45.0 * t2612 * t1893;
    let t7035 = t5109 * t1044 * t1793;
    let t7036 = t1621 * t7035;
    let t7038 = 4.0 / 5.0 * t639 * t7036;
    let t7039 = t837 * t641;
    let t7040 = t7039 * t2592;
    let t7041 = t639 * t7040;
    let t7042 = 8.0 / 27.0 * t7041;
    let t7043 = t5493 * t2597;
    (t7026, t7031, t7033, t7038, t7042, t7043)
}
