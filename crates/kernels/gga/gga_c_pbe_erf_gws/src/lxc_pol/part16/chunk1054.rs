//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1054/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1054<F: Float>(t3221: F, t9550: F, t3220: F, t1123: F, t6396: F, t2255: F, t2253: F, t2277: F, t2312: F, t6477: F, t9033: F, t9037: F, t9039: F, t9041: F, t9042: F, t9539: F, t9540: F, t9545: F, t9549: F) -> (F, F, F, F, F) {
    let t9551 = t3221 * t9550;
    let t9552 = t3220 * t9551;
    let t9555 = t1123 * t6396;
    let t9556 = t2255 * t9555;
    let t9559 = -F::new(7.0) / F::new(1152.0) * t6477 + t9539 - t9033 - t2312 * t9540 / F::new(192.0) + t2277 * t9545 / F::new(768.0) + t9549 - t2253 * t9552 / F::new(768.0) - t2253 * t9556 / F::new(768.0) + t9037 - t9039 - t9041 - t9042;
    (t9551, t9552, t9555, t9556, t9559)
}
