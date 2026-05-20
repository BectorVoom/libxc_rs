//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1563/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1563<F: Float>(t17308: F, t20846: F, t24639: F, t3172: F, t3711: F, t13062: F, t24545: F, t1261: F, t24807: F, t17377: F, t20786: F, t24604: F, t5384: F) -> (F, F, F, F, F, F) {
    let t83851 = t17308 * t20846;
    let t83860 = t3711 * t3172 * t24639;
    let t83863 = t13062 * t3172 * t24545;
    let t83871 = t1261 * t3172 * t24807;
    let t83891 = t17377 * t20786;
    let t83897 = t5384 * t3172 * t24604;
    (t83851, t83860, t83863, t83871, t83891, t83897)
}
