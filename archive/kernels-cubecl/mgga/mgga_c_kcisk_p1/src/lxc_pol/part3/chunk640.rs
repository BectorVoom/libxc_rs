//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 640/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk640<F: Float>(t5014: F, t662: F, t1310: F, t657: F, t718: F, t733: F, t1755: F, t41: F, t5320: F, t739: F, t5330: F, t79: F) -> (F, F, F, F, F, F) {
    let t7242 = t5014 * t662;
    let t7261 = t1310 * t657;
    let t7302 = t733 * t718;
    let t7303 = t41 * t1755;
    let t7310 = t739 * t5320;
    let t7311 = t79 * t5330;
    (t7242, t7261, t7302, t7303, t7310, t7311)
}
