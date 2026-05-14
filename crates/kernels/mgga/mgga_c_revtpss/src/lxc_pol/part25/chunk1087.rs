//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1087/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1087<F: Float>(t2645: F, t837: F, t211: F, t9644: F, t138: F, t785: F, t9302: F, t10818: F, t221: F, t2452: F, t9720: F, t675: F, t886: F, t11006: F, t256: F, t10115: F, t251: F) -> (F, F, F, F, F, F, F, F) {
    let t39620 = t837 * t2645;
    let t39643 = 1.0 / t9644 / t211;
    let t40270 = t138 * t9302 * t785;
    let t40419 = t221 * t10818;
    let t40688 = t9720 * t2452;
    let t41040 = t675 * t886;
    let t41077 = 1.0 / t11006 / t256;
    let t41117 = t10115 * t251;
    (t39620, t39643, t40270, t40419, t40688, t41040, t41077, t41117)
}
