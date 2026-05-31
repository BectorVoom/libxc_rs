//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1018/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1018<F: Float>(t147: F, t39637: F, t41399: F, t10051: F, t754: F, t10053: F, t10002: F, t9838: F, t10050: F, t257: F, t255: F, t2569: F, t10052: F, t2526: F) -> (F, F, F, F, F) {
    let t148 = F::cast_from(10000000.0_f64) <= t147;
    let t41401 = piecewise3::<F>(t148, F::cast_from(0.0_f64), t39637 + t41399);
    let t41402 = t754 * t10051;
    let t41403 = t41402 * t10053;
    let t41405 = t10002 * t9838;
    let t41408 = F::cast_from(1.0_f64) / t10050 / t257;
    let t41409 = t255 * t41408;
    let t41410 = t2569 * t2569;
    let t41411 = t41409 * t41410;
    let t41414 = t10052 * t2569 * t2526;
    (t41401, t41403, t41405, t41411, t41414)
}
