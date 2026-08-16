//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 939/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk939<F: Float>(t22267: F, t4018: F, t6864: F, t9918: F, t3930: F, t6876: F, t6862: F, t72: F, t686: F, t10023: F, t1385: F, t6888: F) -> (F, F, F, F, F) {
    let t22268 = t4018 * t22267;
    let t22285 = t9918 * t6864;
    let t22292 = t3930 * t6876;
    let t22314 = t6862 * t72;
    let t22315 = t22314 * t686;
    let t22316 = t10023 * t22315;
    let t22321 = t1385 * t6888;
    (t22268, t22285, t22292, t22316, t22321)
}
