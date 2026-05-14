//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 823/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk823<F: Float>(t38885: F, t977: F, t1960: F, t2728: F, t3749: F, t2358: F, t39337: F, t12270: F, t2592: F, t13765: F, t4342: F, t1382: F, t2497: F, t3718: F, t40942: F, t40946: F) -> (F, F, F, F, F, F, F, F) {
    let t47097 = t38885 * t977;
    let t47105 = t1960 * t3749 * t2728;
    let t47107 = t39337 * t2358;
    let t47112 = t2592 * t12270;
    let t47114 = t4342 * t13765;
    let t47120 = t1382 * t3718 * t2497;
    let t47126 = 0.15337170381568299871e1 * t40942;
    let t47127 = 0.38342925953920749677e0 * t40946;
    (t47097, t47105, t47107, t47112, t47114, t47120, t47126, t47127)
}
