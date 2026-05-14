//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 719/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk719<F: Float>(t4203: F, t6357: F, t1492: F, t2262: F, t1487: F, t1512: F, t2278: F, t493: F, t2263: F, t4215: F, t3784: F, t4229: F) -> (F, F, F, F, F, F, F) {
    let t6358 = t4203 * t6357;
    let t6360 = t1492 * t2262;
    let t6361 = t1487 * t6360;
    let t6363 = t1512 * t2278;
    let t6364 = t493 * t6363;
    let t6366 = t4215 * t2263;
    let t6368 = t3784 * t4229;
    (t6358, t6360, t6361, t6363, t6364, t6366, t6368)
}
