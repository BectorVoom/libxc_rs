//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 783/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk783<F: Float>(t1927: F, t644: F, t1926: F, t531: F, t7311: F, t1962: F, t198: F, t206: F) -> (F, F, F) {
    let t25163 = t1927 * t644;
    let t25164 = t1926 * t25163;
    let t25190 = t531 * t7311;
    let t25206 = t198 * t206 * t1962;
    (t25164, t25190, t25206)
}
