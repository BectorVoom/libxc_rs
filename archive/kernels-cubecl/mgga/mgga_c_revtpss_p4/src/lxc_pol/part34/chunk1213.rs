//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1213/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1213<F: Float>(t25877: F, t94382: F, t7246: F, t9692: F, t1955: F, t7282: F, t9656: F, t281: F, t555: F, t93238: F, t25917: F, t9303: F) -> (F, F, F, F, F) {
    let t94771 = t94382 * t25877;
    let t94784 = F::cast_from(0.30356481678079769392e-1_f64) * t7246 * t9692;
    let t94823 = t1955 * t7282 * t9656;
    let t94849 = t281 * t93238 * t555;
    let t94854 = F::cast_from(0.26019841438354088051e-2_f64) * t9303 * t25917;
    (t94771, t94784, t94823, t94849, t94854)
}
