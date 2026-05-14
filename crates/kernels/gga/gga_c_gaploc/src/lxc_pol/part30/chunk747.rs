//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 747/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk747<F: Float>(t1865: F, t2581: F, t1445: F, t4371: F, t944: F, t958: F, t2668: F, t4614: F, t2582: F, t1422: F, t6109: F, t787: F) -> (F, F, F, F, F, F, F, F) {
    let t7499 = t2581 * t1865;
    let t7500 = t1445 * t7499;
    let t7503 = t4371 * t944;
    let t7504 = t958 * t7503;
    let t7506 = t4614 * t2668;
    let t7509 = t4614 * t2582;
    let t7512 = t6109 * t1422;
    let t7513 = t787 * t7512;
    (t7499, t7500, t7503, t7504, t7506, t7509, t7512, t7513)
}
