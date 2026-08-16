//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1274/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1274(t37419: f64, t37423: f64, t40303: f64, t40305: f64, t40308: f64, t40315: f64, t42187: f64, t43854: f64, t44140: f64, t44143: f64, t44147: f64, t44150: f64, t44152: f64, t44155: f64, t44158: f64) -> f64 {
    let t44979 = 0.29810146462873361016e-2_f64 * t37419 + t44140 - t44143 + 0.72042316457491791901e-3_f64 * t37423 - t44147 + t44150 - t44152 - t44155 + 0.30487649791575028312e-3_f64 * t43854 - t44158 - 0.7684513755465791136e-2_f64 * t40303 + 0.18446557979282192534e-2_f64 * t40305 + 0.1440846329149835838e-2_f64 * t40308 + t42187 - 0.17347588262831798123e-3_f64 * t40315;
    t44979
}
