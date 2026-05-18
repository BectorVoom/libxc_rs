//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1173/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1173<F: Float>(t39355: F, t39358: F, t39362: F, t39396: F, t39401: F, t39404: F, t39411: F, t43009: F, t43012: F, t43015: F, t43018: F, t43021: F) -> F {
    let t43023 = -F::new(0.14282990759302185292e-1) * t39355 - F::new(0.57131963037208741168e-1) * t39358 - F::new(0.10975748638225852664e0) * t43009 - t39362 - F::new(0.86682217400542685632e-1) * t43012 + F::new(0.2600466522016280569e0) * t43015 + F::new(0.86682217400542685632e-1) * t43018 - F::new(0.32927245914677557992e0) * t43021 + t39396 - t39401 - t39404 + t39411;
    t43023
}
