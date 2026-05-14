//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 761/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk761<F: Float>(t1620: F, t7043: F, t1627: F, t2640: F, t1660: F, t197: F, t5219: F, t572: F, t108: F, t182: F, t267: F, t1764: F, t597: F, t1663: F, t2647: F, t723: F) -> (F, F, F, F, F, F, F, F) {
    let t7045 = 16.0 / 45.0 * t1620 * t7043;
    let t7047 = 16.0 / 135.0 * t1627 * t2640;
    let t7048 = t1660 * t197;
    let t7055 = t5219 * t572;
    let t7061 = t182 * t108;
    let t7062 = t7061 * t267;
    let t7063 = t5219 * t1764;
    let t7068 = t1660 * t597;
    let t7069 = t7068 * t1663;
    let t7074 = 4.0 / 9.0 * t2647 * t723;
    (t7045, t7047, t7048, t7055, t7062, t7063, t7069, t7074)
}
