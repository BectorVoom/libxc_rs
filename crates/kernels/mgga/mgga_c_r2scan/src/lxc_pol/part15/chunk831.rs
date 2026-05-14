//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 831/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk831<F: Float>(t106: F, t797: F, t8299: F, t97: F, t4873: F, t5032: F, t5039: F, t7141: F, t7144: F, t7148: F, t7149: F, t7150: F, t7156: F, t7158: F, t7160: F, t7161: F) -> (F,) {
    let t8302 = t97 * t106 * t8299 * t797;
    let t8303 = t7141 - t7144 + t7148 + t7149 + t7150 - t4873 - t8302 + t7156 + t7158 + t7160 - t5032 - t7161 - t5039;
    (t8303,)
}
