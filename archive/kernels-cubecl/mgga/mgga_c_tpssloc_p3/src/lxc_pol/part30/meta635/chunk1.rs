//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2045/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2045<F: Float>(t87910: F, t1519: F, t212: F, t23171: F, t6554: F, t25040: F, t82074: F, t87712: F, t25193: F, t81591: F, t10143: F, t7540: F) -> (F, F, F, F, F) {
    let t87911 = F::cast_from(0.82246703342411321824e-2_f64) * t87910;
    let t87915 = t23171 * t212 * t1519 * t6554;
    let t87927 = t87712 * t82074 * t25040;
    let t87931 = t81591 * t25193;
    let t87932 = F::cast_from(0.76763589786250567036e-1_f64) * t87931;
    let t87975 = t7540 * t10143;
    (t87911, t87915, t87927, t87932, t87975)
}
