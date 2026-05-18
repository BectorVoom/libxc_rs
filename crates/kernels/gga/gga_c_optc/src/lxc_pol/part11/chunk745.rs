//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 745/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk745<F: Float>(t1244: F, t1871: F, t40: F, t3546: F, t740: F, t1310: F, t2204: F, t4: F) -> (F, F, F, F, F) {
    let t9521 = t1244 * t1871;
    let t9522 = t40 * t9521;
    let t9523 = t3546 * t740;
    let t9527 = t1310 * t2204;
    let t9529 = t1244 * t4;
    (t9521, t9522, t9523, t9527, t9529)
}
