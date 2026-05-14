//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 652/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk652<F: Float>(t716: F, t224: F, t719: F, t1691: F, t695: F) -> (F, F, F, F) {
    let t5265 = t716 * t716;
    let t5266 = 1.0 / t5265;
    let t5267 = t5266 * t224;
    let t5268 = t719 * t719;
    let t5269 = 1.0 / t5268;
    let t5270 = t1691 * t695;
    (t5266, t5267, t5269, t5270)
}
