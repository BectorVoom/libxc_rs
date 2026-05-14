//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1133/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1133<F: Float>(t2209: F, t3739: F, t24081: F, t6853: F, t22851: F, t6181: F, t10346: F, t11210: F, t16677: F, t19: F, t6939: F, t11626: F, t3234: F, t6179: F, t11625: F, t11669: F, t2440: F) -> (F, F, F, F, F, F) {
    let t35811 = t2209 * t3739;
    let t35813 = t24081 * t6853;
    let t35815 = t35813 * t6181 * t22851;
    let t35820 = t10346 * t6939 * t19 * t11210 * t16677;
    let t35823 = t3234 * t6179 * t11626;
    let t35826 = t11625 * t11669 * t2440;
    (t35811, t35813, t35815, t35820, t35823, t35826)
}
