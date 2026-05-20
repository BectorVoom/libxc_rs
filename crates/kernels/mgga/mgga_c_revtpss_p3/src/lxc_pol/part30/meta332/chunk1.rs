//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1339/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1339<F: Float>(t2778: F, t9303: F, t871: F, t9292: F, t2760: F, t72: F, t686: F, t874: F, t251: F, t9646: F, t22: F, t780: F) -> (F, F, F, F, F) {
    let t10969 = F::cast_from(0.26019841438354088051e-2_f64) * t9303 * t2778;
    let t10971 = F::cast_from(0.17073386770573548589e-1_f64) * t9292 * t871;
    let t10972 = t2760 * t72;
    let t10974 = t874 * t10972 * t686;
    let t10981 = t9646 * t251;
    let t10982 = t780 * t22;
    (t10969, t10971, t10974, t10981, t10982)
}
