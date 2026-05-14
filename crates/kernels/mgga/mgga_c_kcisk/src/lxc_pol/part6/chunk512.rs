//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 512/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk512<F: Float>(t1907: F, t2541: F, t718: F, t733: F, t1755: F, t41: F, t5320: F, t739: F, t5330: F, t79: F, t5283: F, t719: F, t6973: F, t740: F, t1871: F, t2558: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7293 = t2541 * t1907;
    let t7302 = t733 * t718;
    let t7303 = t41 * t1755;
    let t7310 = t739 * t5320;
    let t7311 = t79 * t5330;
    let t7315 = t5283 * t718;
    let t7316 = t41 * t719;
    let t7320 = t6973 * t740;
    let t7336 = t2558 * t1871;
    (t7293, t7302, t7303, t7310, t7311, t7315, t7316, t7320, t7336)
}
