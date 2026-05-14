//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1040/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1040<F: Float>(t1091: F, t2665: F, t28746: F, t6317: F, t4973: F, t6318: F, t10409: F, t4965: F, t1212: F, t1234: F, t2862: F, t24980: F, t4969: F, t24989: F, t5225: F, t193: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t31589 = t2665 * t28746 * t1091;
    let t31590 = t6317 * t31589;
    let t31593 = t2665 * t6318 * t4973;
    let t31594 = t6317 * t31593;
    let t31597 = t10409 * t6318 * t4965;
    let t31598 = t6317 * t31597;
    let t31600 = t1234 * t1212;
    let t31602 = t2862 * t6318 * t31600;
    let t31603 = t24980 * t31602;
    let t31605 = t2665 * t6318 * t4969;
    let t31606 = t6317 * t31605;
    let t31608 = t24989 * t5225;
    let t31609 = t193 * t31608;
    (t31589, t31590, t31593, t31594, t31597, t31598, t31600, t31602, t31603, t31605, t31606, t31608, t31609)
}
