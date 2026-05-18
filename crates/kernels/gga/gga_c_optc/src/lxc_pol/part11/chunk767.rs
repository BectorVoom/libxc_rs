//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 767/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk767<F: Float>(t1464: F, t2934: F, t1446: F, t3017: F, t1519: F, t7878: F, t1133: F, t1523: F, t3169: F, t116: F, t3241: F, t3242: F) -> (F, F, F, F, F, F) {
    let t12268 = t1464 * t2934;
    let t12366 = t1446 * t3017;
    let t12489 = t7878 * t1519;
    let t12490 = t1133 * t12489;
    let t12522 = t1523 * t3169;
    let t12567 = t3241 * t3242 * t116;
    (t12268, t12366, t12489, t12490, t12522, t12567)
}
