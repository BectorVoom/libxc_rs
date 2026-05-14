//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1301/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1301<F: Float>(t17807: F, t489: F, t3759: F, t5230: F, t1811: F, t3601: F, t3769: F, t16695: F, t17454: F, t473: F, t5412: F, t1214: F, t1269: F, t1287: F, t5284: F, t17633: F, t5458: F) -> (F, F, F, F, F, F, F, F) {
    let t17808 = t489 * t17807;
    let t17811 = t3759 * t5230;
    let t17814 = t1811 * t3601;
    let t17815 = t17814 * t3769;
    let t17818 = t16695 * t17454;
    let t17821 = t473 * t5412;
    let t17822 = t17821 * t1214;
    let t17826 = t1269 * t5284 * t1287;
    let t17829 = t17633 * t5458;
    (t17808, t17811, t17814, t17815, t17818, t17822, t17826, t17829)
}
