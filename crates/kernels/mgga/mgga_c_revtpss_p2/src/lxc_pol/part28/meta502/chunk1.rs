//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1892/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1892<F: Float>(t26123: F, t670: F, t572: F, t2371: F, t7330: F, t117: F, t25832: F, t1461: F, t2040: F, t26106: F, t26115: F, t26117: F, t26119: F, t26122: F, t4162: F, t4165: F, t573: F, t7324: F) -> (F, F, F, F) {
    let t26124 = t26123 * t670;
    let t26126 = F::new(12.0) * t572 * t26124;
    let t26127 = t7330 * t2371;
    let t26129 = F::new(6.0) * t572 * t26127;
    let t26130 = t117 * t25832;
    let t26132 = F::new(3.0) * t572 * t26130;
    let t26133 = F::new(6.0) * t1461 * t7324 + F::new(6.0) * t2040 * t4162 + F::new(3.0) * t2040 * t4165 + t26106 * t573 + t26115 + t26117 + t26119 + t26122 + t26126 + t26129 + t26132;
    (t26124, t26127, t26130, t26133)
}
