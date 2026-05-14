//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1012/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1012<F: Float>(t23021: F, t2558: F, t9647: F, t1841: F, t9652: F, t1843: F, t21476: F, t22045: F, t2547: F, t279: F, t481: F) -> (F, F, F, F) {
    let t29354 = 0.64087718584518535698e-3 * t9647 * t23021 * t2558;
    let t29434 = 0.34180116578409885706e-2 * t1841 * t9652;
    let t29437 = 0.1281754371690370714e-2 * t21476 * t1843 * t22045;
    let t29439 = t481 * t2547 * t279;
    (t29354, t29434, t29437, t29439)
}
