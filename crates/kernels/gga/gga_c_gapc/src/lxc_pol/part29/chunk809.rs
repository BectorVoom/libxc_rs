//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 809/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk809<F: Float>(t1084: F, t8986: F, t2562: F, t2636: F, t8619: F, t3327: F, t7191: F, t2316: F, t2982: F, t3391: F, t2300: F, t3387: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9414 = t1084 * t8986;
    let t9415 = t2636 * t2562;
    let t9416 = t9414 * t9415;
    let t9418 = t1084 * t8619;
    let t9419 = t3327 * t7191;
    let t9420 = t9418 * t9419;
    let t9422 = t2982 * t2316;
    let t9423 = t3391 * t9422;
    let t9425 = t2982 * t2300;
    let t9426 = t3387 * t9425;
    (t9414, t9415, t9416, t9418, t9419, t9420, t9422, t9423, t9425, t9426)
}
