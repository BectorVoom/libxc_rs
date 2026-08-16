//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 789/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk789<F: Float>(t1445: F, t7487: F, t5750: F, t935: F, t1865: F, t7227: F, t2581: F, t4371: F, t944: F, t958: F, t2668: F, t4614: F) -> (F, F, F, F, F, F, F, F) {
    let t7488 = t1445 * t7487;
    let t7491 = t5750 * t935;
    let t7492 = t7491 * t1865;
    let t7493 = t1445 * t7492;
    let t7496 = t1445 * t7227;
    let t7499 = t2581 * t1865;
    let t7500 = t1445 * t7499;
    let t7503 = t4371 * t944;
    let t7504 = t958 * t7503;
    let t7506 = t4614 * t2668;
    (t7488, t7493, t7496, t7499, t7500, t7503, t7504, t7506)
}
