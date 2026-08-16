//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1316/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1316<F: Float>(t1062: F, t3728: F, t6935: F, t2209: F, t3739: F, t24081: F, t6853: F, t22851: F, t6181: F, t10346: F, t11210: F, t16677: F, t19: F, t6939: F) -> (F, F, F, F, F) {
    let t35809 = t1062 * t3728 * t6935;
    let t35811 = t2209 * t3739;
    let t35813 = t24081 * t6853;
    let t35815 = t35813 * t6181 * t22851;
    let t35820 = t10346 * t6939 * t19 * t11210 * t16677;
    (t35809, t35811, t35813, t35815, t35820)
}
