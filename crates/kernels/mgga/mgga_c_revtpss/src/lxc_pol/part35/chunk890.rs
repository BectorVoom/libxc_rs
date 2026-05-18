//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 890/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk890<F: Float>(t187: F, t22789: F, t13621: F, t13630: F, t13633: F, t22764: F, t22765: F, t22766: F, t22768: F, t22791: F, t9394: F, t9396: F, t9409: F, t9412: F) -> (F, F, F, F, F) {
    let t22919 = F::new(0.19751673498613801407e-1) * t22789 * t187;
    let t22920 = F::new(24.0) * t13621;
    let t22921 = F::new(0.35089341735807877242e1) * t13630;
    let t22922 = F::new(3.0) * t13633;
    let t22923 = -t22764 - t22765 + t22766 - t22768 + t22791 + t22919 + t9394 - t22920 - t9396 + t22921 + t22922 + t9409 - t9412;
    (t22919, t22920, t22921, t22922, t22923)
}
