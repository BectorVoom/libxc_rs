//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1100/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1100<F: Float>(t2439: F, t25335: F, t212: F, t7048: F, t780: F, t689: F, t231: F, t836: F, t7076: F, t1949: F, t2645: F, t7014: F, t887: F) -> (F, F, F, F, F, F, F, F) {
    let t25337 = F::new(0.65049603595885220126e-3) * t2439 * t25335;
    let t25338 = t212 * t7048;
    let t25339 = t25338 * t780;
    let t25340 = t689 * t25339;
    let t25343 = t7048 * t836 * t231;
    let t25344 = t7076 * t25343;
    let t25348 = t1949 * t2645 * t231;
    let t25349 = t7076 * t25348;
    let t25352 = t7014 * t887;
    (t25337, t25338, t25339, t25340, t25344, t25348, t25349, t25352)
}
