//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 872/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk872<F: Float>(t246: F, t4873: F, t5032: F, t5039: F, t6036: F, t6039: F, t6047: F, t7028: F, t7156: F, t7158: F, t7160: F, t7161: F) -> F {
    let t7910 = -t4873 + F::cast_from(0.285764e-1_f64) * t6036 + F::cast_from(0.571528e-1_f64) * t6039 + t6047 - F::cast_from(0.285764e-1_f64) * t246 * t7028 + t7156 + t7158 + t7160 - t5032 - t7161 - t5039;
    t7910
}
