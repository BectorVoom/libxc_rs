//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1558/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1558<F: Float>(t12772: F, t24786: F, t3625: F, t17572: F, t21188: F, t13052: F, t24667: F, t3172: F, t12916: F, t24705: F, t3718: F, t1222: F, t17240: F, t24244: F) -> (F, F, F, F, F) {
    let t83435 = t3625 * t12772 * t24786;
    let t83462 = t17572 * t21188;
    let t83485 = t13052 * t3172 * t24667;
    let t83490 = t3718 * t12916 * t24705;
    let t83504 = t1222 * t17240 * t24244;
    (t83435, t83462, t83485, t83490, t83504)
}
