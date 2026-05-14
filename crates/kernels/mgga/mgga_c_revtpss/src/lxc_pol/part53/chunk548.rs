//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 548/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk548<F: Float>(t3357: F, t3358: F, t5044: F, t5049: F, t5054: F, t5058: F, t422: F, t1130: F, t1719: F, t1151: F, t1733: F, t3379: F, t1149: F, t3384: F, t1723: F, t3390: F) -> (F, F, F, F, F) {
    let t5060 = t3357 - 0.5936111111111111111e-2 * t3358 - 0.5936111111111111111e-2 * t5044 - 0.11872222222222222222e-1 * t5049 + 0.35616666666666666666e-1 * t5054 + 0.17808333333333333333e-1 * t5058;
    let t5062 = 0.621814e-1 * t5060 * t422;
    let t5063 = t1719 * t1130;
    let t5065 = 1.0 * t5063 * t1151;
    let t5067 = 1.0 * t3379 * t1733;
    let t5068 = t1733 * t1149;
    let t5070 = 2.0 * t3384 * t5068;
    let t5071 = t3390 * t1723;
    (t5062, t5065, t5067, t5070, t5071)
}
