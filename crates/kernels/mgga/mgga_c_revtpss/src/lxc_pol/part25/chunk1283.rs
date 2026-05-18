//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1283/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1283<F: Float>(t11240: F, t11244: F, t11627: F, t25503: F, t11273: F, t25508: F, t25526: F, t3173: F, t11263: F, t7122: F, t11762: F, t7111: F, sigma0: F) -> (F, F, F, F, F, F) {
    let t93789 = t11240 * t11627 * sigma0 * t11244;
    let t93793 = t11240 * t25503 * t11244;
    let t93796 = t11273 * t25508;
    let t93799 = t25526 * t3173;
    let t93801 = t7122 * t11263;
    let t93813 = t7111 * t11762;
    (t93789, t93793, t93796, t93799, t93801, t93813)
}
