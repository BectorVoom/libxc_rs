//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 778/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk778<F: Float>(t267: F, t7061: F, t1764: F, t5219: F, t418: F, t7056: F, t1660: F, t597: F, t1663: F, t2647: F, t723: F, t2650: F, t4985: F, t4993: F, t4996: F, t4987: F, t7026: F, t7031: F, t7033: F, t7038: F, t7042: F, t7045: F, t7047: F, t7054: F, t7060: F) -> (F, F, F, F, F, F, F) {
    let t7062 = t7061 * t267;
    let t7063 = t5219 * t1764;
    let t7064 = t7056 * t418;
    let t7065 = t7063 * t7064;
    let t7067 = 16.0 / 45.0 * t7062 * t7065;
    let t7068 = t1660 * t597;
    let t7069 = t7068 * t1663;
    let t7070 = t7069 * t7064;
    let t7072 = 8.0 / 27.0 * t7062 * t7070;
    let t7074 = 4.0 / 9.0 * t2647 * t723;
    let t7075 = t2650 * t723;
    let t7077 = 8.0 / 45.0 * t4985;
    let t7079 = 16.0 / 405.0 * t4993;
    let t7080 = 16.0 / 135.0 * t4996;
    let t7081 = -t7026 + t7031 - t7033 - t7038 + t7042 - t7045 + t7047 + t7054 - t7060 + t7067 - t7072 + t7074 + 4.0 / 9.0 * t7075 + t7077 - 2.0 / 45.0 * t4987 - t7079 - t7080;
    (t7062, t7067, t7072, t7077, t7079, t7080, t7081)
}
