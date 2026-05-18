//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1040/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1040<F: Float>(t694: F, t8379: F, t1674: F, t8373: F, t10409: F, t1679: F, t560: F, t2354: F, t469: F, t301: F, t11883: F, t624: F) -> (F, F, F, F, F, F) {
    let t36575 = F::new(6.0) * t694 * t8379;
    let t36592 = F::new(12.0) * t1674 * t8373;
    let t36601 = F::new(2.0) * t1679 * t10409 * t560;
    let t36602 = t2354 * t469;
    let t36605 = F::new(6.0) * t694 * t36602 * t301;
    let t36610 = t624 * t11883;
    (t36575, t36592, t36601, t36602, t36605, t36610)
}
