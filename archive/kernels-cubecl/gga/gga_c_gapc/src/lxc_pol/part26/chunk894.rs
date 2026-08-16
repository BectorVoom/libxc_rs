//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 894/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk894<F: Float>(t10102: F, t1058: F, t2153: F, t996: F, t3206: F, t731: F, t283: F, t492: F, t3193: F, t2786: F, t282: F, t61: F) -> (F, F, F, F, F, F) {
    let t10371 = t10102 * t1058;
    let t10373 = t996 * t2153;
    let t10374 = t10373 * t1058;
    let t10376 = t731 * t3206;
    let t10378 = t492 * t283;
    let t10379 = t10378 * t3193;
    let t10381 = t2786 * t282;
    let t10382 = t61 * t10381;
    (t10371, t10373, t10374, t10376, t10379, t10382)
}
