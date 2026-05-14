//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1033/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1033<F: Float>(t13892: F, t13902: F, t13907: F, t13911: F, t13914: F, t13917: F, t1392: F, t1395: F, t1877: F, t1879: F, t4045: F, t4050: F, t4053: F, t539: F, t541: F, t5644: F, t5650: F, t5652: F, t5655: F) -> (F,) {
    let t13920 = -t13892 * t541 - 24.0 * t13902 * t5652 + 60.0 * t13907 * t5650 - 24.0 * t13911 * t5650 - 12.0 * t13914 * t5650 + 3.0 * t13917 * t539 + 6.0 * t1392 * t5655 + 6.0 * t1395 * t5644 - 12.0 * t1877 * t4050 + 3.0 * t1877 * t4053 + 3.0 * t1879 * t4045;
    (t13920,)
}
