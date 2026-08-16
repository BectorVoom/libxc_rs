//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1053/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1053<F: Float>(t12153: F, t972: F, t1125: F, t9375: F, t3449: F, t3565: F, t3832: F, t7056: F, t11046: F, t3268: F, t3265: F, t3622: F) -> (F, F, F, F, F, F) {
    let t12154 = t12153 * t972;
    let t12155 = t9375 * t1125;
    let t12156 = t3565 * t3449;
    let t12158 = F::cast_from(2.0_f64) * t7056 * t3832;
    let t12161 = F::cast_from(2.0_f64) * t11046 * t3268;
    let t12162 = t3265 * t3622;
    (t12154, t12155, t12156, t12158, t12161, t12162)
}
