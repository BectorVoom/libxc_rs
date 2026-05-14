//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 808/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk808<F: Float>(t3253: F, t6203: F, t1154: F, t6455: F, t3261: F, t6416: F, t3291: F, t254: F, t6: F, t6469: F, t2323: F, t3268: F, t1113: F, t904: F) -> (F, F, F, F, F, F, F) {
    let t9447 = 7.0 / 288.0 * t6203 * t3253;
    let t9457 = t6455 * t1154;
    let t9464 = 7.0 / 576.0 * t6416 * t3261;
    let t9474 = 7.0 / 1152.0 * t6416 * t3291;
    let t9482 = t254 * t6 * t6469;
    let t9498 = 7.0 / 576.0 * t2323 * t3268;
    let t9499 = t904 * t1113;
    (t9447, t9457, t9464, t9474, t9482, t9498, t9499)
}
