//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1184/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1184<F: Float>(t1518: F, t1931: F, t4147: F, t7933: F, t11239: F, t3268: F, t211: F, t9644: F, t138: F, t785: F, t9302: F, t2452: F, t9720: F) -> (F, F, F, F, F, F) {
    let t33602 = t1931 * t1518;
    let t33651 = t4147 * t7933;
    let t36870 = t11239 * t3268;
    let t39643 = F::cast_from(1.0_f64) / t9644 / t211;
    let t40270 = t138 * t9302 * t785;
    let t40688 = t9720 * t2452;
    (t33602, t33651, t36870, t39643, t40270, t40688)
}
