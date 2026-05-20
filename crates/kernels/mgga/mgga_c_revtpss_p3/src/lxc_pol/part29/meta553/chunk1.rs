//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1893/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1893<F: Float>(t2453: F, t26264: F, t9676: F, t26072: F, t26271: F, t26231: F, t94921: F, t10073: F, t1444: F, t2102: F, t25929: F, t7496: F, t9692: F) -> (F, F, F, F, F, F) {
    let t96515 = t2453 * t26264;
    let t96516 = t96515 * t9676;
    let t96527 = t26072 * t26271;
    let t96542 = t94921 * t26231;
    let t96546 = t10073 * t25929 * t2102 * t1444;
    let t96549 = F::cast_from(0.30356481678079769392e-1_f64) * t7496 * t9692;
    (t96515, t96516, t96527, t96542, t96546, t96549)
}
