//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1091/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1091<F: Float>(t4147: F, t7933: F, t1518: F, t2126: F, t11239: F, t3736: F, t211: F, t9644: F, t138: F, t785: F, t9302: F, t2452: F, t9720: F, t11006: F, t256: F, t10115: F, t251: F) -> (F, F, F, F, F, F, F, F) {
    let t33651 = t4147 * t7933;
    let t34446 = t2126 * t1518;
    let t37885 = t11239 * t3736;
    let t39643 = 1.0 / t9644 / t211;
    let t40270 = t138 * t9302 * t785;
    let t40688 = t9720 * t2452;
    let t41077 = 1.0 / t11006 / t256;
    let t41117 = t10115 * t251;
    (t33651, t34446, t37885, t39643, t40270, t40688, t41077, t41117)
}
