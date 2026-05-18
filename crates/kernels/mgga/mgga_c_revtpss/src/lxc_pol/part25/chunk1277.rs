//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1277/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1277<F: Float>(t25490: F, t3215: F, t11951: F, t7117: F, t11643: F, t25522: F, t12009: F, t25505: F, t25531: F, t800: F, t25539: F, t3244: F) -> (F, F, F, F, F, F) {
    let t93683 = t25490 * t3215;
    let t93685 = t7117 * t11951;
    let t93687 = t25522 * t11643;
    let t93689 = t25505 * t12009;
    let t93691 = t25531 * t800;
    let t93694 = t25539 * t3244;
    (t93683, t93685, t93687, t93689, t93691, t93694)
}
