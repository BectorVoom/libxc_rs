//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2410/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2410<F: Float>(t11627: F, t42859: F, t342: F, t12077: F, t989: F, t12153: F, t3057: F, t1071: F, t11200: F, t3494: F, t3519: F, t13026: F, t240: F) -> (F, F, F, F, F, F, F) {
    let t43536 = t42859 * t11627;
    let t43537 = t342 * t43536;
    let t43574 = t989 * t12077;
    let t43598 = t3057 * t12153;
    let t43637 = t11200 * t1071;
    let t43752 = F::cast_from(1.0_f64) / t3519 / t3494;
    let t43764 = t240 * t13026;
    (t43536, t43537, t43574, t43598, t43637, t43752, t43764)
}
