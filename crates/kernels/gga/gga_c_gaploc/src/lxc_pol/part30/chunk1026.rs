//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1026/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1026<F: Float>(t2464: F, t2465: F, t2487: F, t6428: F, t3178: F, t4625: F, t1407: F, t9279: F, t9555: F, t3193: F, t4634: F, t28229: F, t3192: F, t574: F, t1641: F, t9421: F) -> (F, F, F, F, F, F, F) {
    let t30378 = 0.17041300423964777634e0 * t2487 * t2464 * t2465 * t6428;
    let t30379 = t4625 * t3178;
    let t30380 = 0.38342925953920749676e0 * t30379;
    let t30381 = t1407 * t9279;
    let t30382 = 0.76685851907841499352e0 * t30381;
    let t30387 = t1407 * t9555;
    let t30388 = 0.1022478025437886658e1 * t30387;
    let t30404 = t4634 * t3193;
    let t30542 = t574 * t28229 * t3192;
    let t30546 = t1641 * t9421;
    (t30378, t30380, t30382, t30388, t30404, t30542, t30546)
}
