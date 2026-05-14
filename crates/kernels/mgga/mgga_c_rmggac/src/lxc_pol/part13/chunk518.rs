//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 518/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk518<F: Float>(t131: F, t1327: F, t640: F, t7323: F, t2012: F, t935: F, t2010: F, t938: F, t1303: F, t20: F, t2018: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7324 = t131 * t1327;
    let t7325 = t640 * t7324;
    let t7326 = t7323 * t7325;
    let t7328 = t2012 * t935;
    let t7329 = t2010 * t7328;
    let t7331 = t2012 * t938;
    let t7332 = t2010 * t7331;
    let t7334 = t1303 * t20;
    let t7335 = t7334 * t2018;
    (t7324, t7325, t7326, t7328, t7329, t7331, t7332, t7334, t7335)
}
