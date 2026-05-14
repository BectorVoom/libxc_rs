//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 950/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk950<F: Float>(t2319: F, t3295: F, t1123: F, t6303: F, t2255: F, t1105: F, t904: F, t2258: F, t1153: F, t9521: F, t8827: F, t3223: F, t2272: F, t3252: F, t2253: F, t2312: F, t6275: F, t6628: F, t6637: F, t6656: F, t9142: F, t9143: F, t9145: F, t9174: F) -> (F, F, F, F, F, F, F, F) {
    let t9601 = 7.0 / 1152.0 * t2319 * t3295;
    let t9603 = t1123 * t6303;
    let t9604 = t2255 * t9603;
    let t9607 = t1105 * param_a_c;
    let t9608 = t904 * t9607;
    let t9609 = t9608 * t2258;
    let t9612 = t1153 * t9521;
    let t9615 = t904 * t8827;
    let t9616 = t9615 * t3223;
    let t9619 = t3252 * t2272;
    let t9623 = -t9601 - t9142 - t9143 - t9145 - 7.0 / 288.0 * t6628 - t2253 * t9604 / 768.0 + t6275 * t9609 / 96.0 + t6637 * t9612 / 768.0 - t6637 * t9616 / 384.0 + t9174 - t2312 * t9619 / 384.0 - 35.0 / 1152.0 * t6656;
    (t9603, t9604, t9607, t9609, t9612, t9616, t9619, t9623)
}
