//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 811/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk811<F: Float>(t213: F, t7274: F, t116: F, t7002: F, t10301: F, t7565: F, t38: F, t7574: F, t2247: F, t2282: F, t55: F, t10309: F) -> (F, F, F, F, F, F) {
    let t26084 = t213 * t7274;
    let t26123 = t116 * t7002;
    let t26749 = t10301 * t7565;
    let t26754 = t38 * t7574;
    let t26755 = t2247 * t26754;
    let t26776 = t55 * t2282;
    let t26792 = t10309 * t7565;
    (t26084, t26123, t26749, t26755, t26776, t26792)
}
