//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1564/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1564<F: Float>(t268: F, t404: F, t7021: F, t1123: F, t2435: F) -> (F, F, F) {
    let t12295 = t268 * t7021 * t404;
    let t12296 = F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t12295;
    let t12297 = t2435 * t1123;
    (t12295, t12296, t12297)
}
