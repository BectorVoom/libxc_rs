//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1019/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1019<F: Float>(t1949: F, t2828: F, t7071: F, t2470: F, t7059: F, t7064: F, t785: F, t780: F, t2439: F, t212: F, t7048: F, t689: F, t231: F, t836: F, t7076: F, t2645: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t25325 = t1949 * t2828;
    let t25326 = t7071 * t25325;
    let t25331 = t7059 * t2470;
    let t25333 = 0.17135234354032049604e-1 * t7064 * t25331;
    let t25334 = t785 * t1949;
    let t25335 = t25334 * t780;
    let t25337 = 0.65049603595885220126e-3 * t2439 * t25335;
    let t25338 = t212 * t7048;
    let t25339 = t25338 * t780;
    let t25340 = t689 * t25339;
    let t25343 = t7048 * t836 * t231;
    let t25344 = t7076 * t25343;
    let t25348 = t1949 * t2645 * t231;
    (t25325, t25326, t25331, t25333, t25334, t25335, t25337, t25338, t25339, t25340, t25344, t25348)
}
