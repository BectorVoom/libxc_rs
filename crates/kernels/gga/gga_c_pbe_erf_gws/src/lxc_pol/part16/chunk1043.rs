//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1043/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1043<F: Float>(t2257: F, t3111: F, t2255: F, t2279: F, t9364: F, t1112: F, t4394: F, t6665: F, t3253: F, t6203: F, t904: F, t9080: F, t916: F) -> (F, F, F, F, F, F, F) {
    let t9433 = t3111 * t2257;
    let t9434 = t2255 * t9433;
    let t9438 = t2255 * t9364 * t2279;
    let t9441 = t1112 * t4394;
    let t9443 = t2255 * t9441 * t6665;
    let t9447 = F::new(7.0) / F::new(288.0) * t6203 * t3253;
    let t9449 = t916 * t904 * t9080;
    (t9433, t9434, t9438, t9441, t9443, t9447, t9449)
}
