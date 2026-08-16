//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2056/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2056<F: Float>(t25192: F, t81651: F, t82074: F, t225: F, t25220: F, t25054: F, t23030: F, t25205: F, t23164: F, t7479: F, t82133: F, t23204: F, t25216: F, t6562: F) -> (F, F, F, F, F, F) {
    let t87835 = t81651 * t82074 * t25192;
    let t87836 = F::cast_from(0.16449340668482264365e-1_f64) * t87835;
    let t87837 = t25220 * t225;
    let t87873 = t81651 * t82074 * t25054;
    let t87874 = F::cast_from(0.16449340668482264365e-1_f64) * t87873;
    let t87898 = t23030 * t25205;
    let t87901 = t23164 * t82133 * t7479;
    let t87902 = F::cast_from(0.16449340668482264365e-1_f64) * t87901;
    let t87910 = t6562 * t23204 * t25216;
    (t87836, t87837, t87874, t87898, t87902, t87910)
}
