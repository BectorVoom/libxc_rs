//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 774/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk774<F: Float>(t22: F, t4616: F, t326: F, t262: F, t265: F, t7835: F, t876: F, t2078: F, t26: F, t3814: F, t36: F, t2064: F, t839: F) -> (F, F, F, F, F, F) {
    let t35928 = t4616 * t22;
    let t35929 = t326 * t35928;
    let t35937 = t7835 * t262 * t265 * t876;
    let t35959 = t2078 * t26;
    let t35960 = t3814 * t35959;
    let t35972 = t4616 * t36;
    let t35979 = t2064 * t839;
    (t35929, t35937, t35959, t35960, t35972, t35979)
}
