//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 855/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk855<F: Float>(t3128: F, t759: F, t761: F, t246: F, t4721: F, t4901: F, t4964: F, t4967: F, t4972: F, t7861: F, t8552: F, t8555: F, t8556: F, t9005: F) -> F {
    let t9040 = t759 * t3128 * t761;
    let t9044 = t8552 - t4901 + t8555 + t7861 + F::new(0.285764e-1) * t9040 - F::new(0.285764e-1) * t246 * t9005 - t4721 + t4964 - t4967 - t8556 - t4972;
    t9044
}
