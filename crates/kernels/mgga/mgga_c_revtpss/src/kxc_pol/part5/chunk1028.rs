//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1028/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1028<F: Float>(t1146: F, t2439: F, t3361: F, t57: F, t268: F, t404: F, t7021: F, t1123: F, t2435: F) -> (F, F, F, F, F) {
    let t12261 = t2439 * t1146;
    let t12267 = t3361 * t57;
    let t12268 = F::new(1.0) / t12267;
    let t12295 = t268 * t7021 * t404;
    let t12296 = F::new(28.0) / F::new(27.0) * t12295;
    let t12297 = t2435 * t1123;
    (t12261, t12268, t12295, t12296, t12297)
}
