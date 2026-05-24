//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 537/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk537<F: Float>(t1692: F, t2399: F, t2469: F, t4826: F, t1907: F, t2541: F, t718: F, t733: F, t1755: F, t41: F, t5320: F, t739: F) -> (F, F, F, F, F, F) {
    let t7278 = t2399 * t1692;
    let t7283 = t2469 * t4826;
    let t7293 = t2541 * t1907;
    let t7302 = t733 * t718;
    let t7303 = t41 * t1755;
    let t7310 = t739 * t5320;
    (t7278, t7283, t7293, t7302, t7303, t7310)
}
