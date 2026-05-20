//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2042/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2042<F: Float>(t12020: F, t7121: F, t3223: F, t7131: F, t1033: F, t11266: F, t7120: F, t25526: F, t3173: F, t11263: F, t7122: F, t11762: F, t7111: F) -> (F, F, F, F, F, F) {
    let t93761 = t12020 * t7121;
    let t93764 = t3223 * t7131;
    let t93774 = t1033 * t7120 * t11266;
    let t93799 = t25526 * t3173;
    let t93801 = t7122 * t11263;
    let t93813 = t7111 * t11762;
    (t93761, t93764, t93774, t93799, t93801, t93813)
}
