//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 758/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk758<F: Float>(t338: F, t830: F, t352: F, t739: F, t4793: F, t669: F, t2157: F, t4685: F, t131: F, t1338: F, t2019: F, t640: F, t7764: F) -> (F, F, F, F, F, F, F) {
    let t35589 = t338 * t830;
    let t35590 = t35589 * t352;
    let t35591 = t739 * t35590;
    let t35593 = t4793 * t669;
    let t35594 = t4685 * t2157;
    let t35604 = t131 * t1338;
    let t35607 = t2019 * t7764 * t640 * t35604;
    (t35589, t35590, t35591, t35593, t35594, t35604, t35607)
}
