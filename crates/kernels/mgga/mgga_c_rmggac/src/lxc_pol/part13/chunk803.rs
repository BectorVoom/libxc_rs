//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 803/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk803<F: Float>(t7255: F, t9159: F, t1614: F, t1970: F, t1971: F, t209: F, t476: F, t511: F, t30900: F, t35972: F, t739: F, t36292: F, t5888: F, t118: F, t2001: F, t2281: F, t495: F) -> (F, F, F, F, F) {
    let t39979 = t7255 * t9159;
    let t39985 = t1970 * t1971 * t511 * t1614 * t476 * t209;
    let t39994 = t739 * t35972 * t30900;
    let t39997 = t739 * t36292 * t5888;
    let t40001 = t2001 * t118 * t2281 * t495;
    (t39979, t39985, t39994, t39997, t40001)
}
