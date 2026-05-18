//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1189/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1189<F: Float>(t39628: F, t39630: F, t39635: F, t39640: F, t39673: F, t41478: F, t41480: F, t43195: F, t43200: F, t43203: F, t43205: F, t43209: F) -> F {
    let t43211 = F::new(0.13099107994629972538e-1) * t43195 + t39628 + t39630 - F::new(0.25426783770825854452e1) * t39635 - t41478 - F::new(0.32927245914677557992e-1) * t39640 + t41480 + F::new(0.13099107994629972538e-1) * t43200 - F::new(0.87327386630866483584e-2) * t43203 - t39673 - F::new(0.13099107994629972538e-1) * t43205 + F::new(0.65495539973149862688e-2) * t43209;
    t43211
}
