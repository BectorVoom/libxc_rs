//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 452/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk452<F: Float>(t166: F, t2049: F, t759: F, t244: F, t757: F, t158: F, t761: F) -> (F, F, F, F, F) {
    let t2050 = t166 * t2049;
    let t2052 = F::cast_from(0.285764e-1_f64) * t759 * t2050;
    let t2053 = t757 * t244;
    let t2054 = F::cast_from(1.0_f64) / t2053;
    let t2055 = t2054 * t158;
    let t2056 = t761 * t761;
    (t2050, t2052, t2054, t2055, t2056)
}
