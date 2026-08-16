//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 731/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk731<F: Float>(t2884: F, t8390: F, t1412: F, t472: F, t144: F, t653: F, t1419: F, t152: F, t200: F, t4296: F, t1603: F, t2957: F) -> (F, F, F, F, F) {
    let t8391 = t2884 * t8390;
    let t8393 = t1412 * t472;
    let t8394 = t653 * t144;
    let t8396 = t8394 * t152 * t1419;
    let t8397 = t8393 * t8396;
    let t8399 = t4296 * t200;
    let t8400 = t8399 * t1603;
    let t8401 = t2957 * t8400;
    (t8391, t8394, t8397, t8399, t8401)
}
