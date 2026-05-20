//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2089/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2089<F: Float>(t1455: F, t8249: F, t116: F, t29421: F, t10301: F, t29411: F, t2247: F, t29362: F, t38: F, t10309: F, t60224: F, t7565: F) -> (F, F, F, F, F, F) {
    let t104094 = F::new(2.0) * t1455 * t8249;
    let t104115 = t29421 * t116;
    let t104181 = t10301 * t29411;
    let t104185 = t2247 * t38 * t29362;
    let t104203 = t10309 * t29411;
    let t104208 = t60224 * t7565;
    (t104094, t104115, t104181, t104185, t104203, t104208)
}
