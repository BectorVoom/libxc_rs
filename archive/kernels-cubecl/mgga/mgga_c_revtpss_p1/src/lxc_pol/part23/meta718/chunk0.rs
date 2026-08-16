//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2477/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2477<F: Float>(t48326: F, t47149: F, t3863: F, t5569: F, t3860: F, t5571: F, t9419: F, t1882: F, t4010: F, t2682: F, t4000: F, t5677: F, t820: F) -> (F, F, F, F, F, F, F) {
    let t48327 = F::cast_from(24.0_f64) * t48326;
    let t48330 = F::cast_from(12.0_f64) * t47149;
    let t48331 = t3863 * t5569;
    let t48332 = F::cast_from(96.0_f64) * t48331;
    let t48333 = t3860 * t5569;
    let t48334 = F::cast_from(36.0_f64) * t48333;
    let t48335 = t5571 * t9419;
    let t48455 = t4010 * t1882;
    let t48486 = t820 * t4000 * t2682 * t5677;
    (t48327, t48330, t48332, t48334, t48335, t48455, t48486)
}
