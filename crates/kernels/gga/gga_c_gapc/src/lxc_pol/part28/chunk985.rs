//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 985/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk985<F: Float>(t126: F, t932: F, t1038: F, t11925: F, t16826: F, t19: F, t7877: F, t15615: F, t3327: F, t17713: F, t23466: F, t8676: F, t3406: F, t8133: F, t2579: F, t3412: F) -> (F, F, F, F, F, F, F) {
    let t30153 = t932 * t126;
    let t30158 = t11925 * t1038 * t7877 * t19 * t16826;
    let t30167 = t3327 * t15615;
    let t30187 = t3327 * t17713;
    let t30288 = t8676 * t23466;
    let t30324 = t3406 * t8133;
    let t30325 = t2579 * t3412 * t30324;
    (t30153, t30158, t30167, t30187, t30288, t30324, t30325)
}
