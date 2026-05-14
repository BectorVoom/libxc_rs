//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 770/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk770<F: Float>(t25334: F, t780: F, t2439: F, t212: F, t7048: F, t689: F, t7014: F, t887: F, t7049: F, t786: F, t789: F, t2471: F, t7018: F, t25331: F, t7058: F, t25309: F, t7063: F) -> (F, F, F, F, F, F, F) {
    let t25335 = t25334 * t780;
    let t25337 = 0.65049603595885220126e-3 * t2439 * t25335;
    let t25338 = t212 * t7048;
    let t25339 = t25338 * t780;
    let t25340 = t689 * t25339;
    let t25352 = t7014 * t887;
    let t25353 = t689 * t25352;
    let t25355 = t786 * t7049;
    let t25356 = t25355 * t789;
    let t25362 = 0.13009920719177044025e-1 * t7018 * t2471;
    let t25364 = 0.96373646535613327357e-2 * t7058 * t25331;
    let t25365 = t7063 * t25309;
    (t25337, t25340, t25353, t25356, t25362, t25364, t25365)
}
