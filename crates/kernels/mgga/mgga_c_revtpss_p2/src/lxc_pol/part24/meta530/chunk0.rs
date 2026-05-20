//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1566/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1566<F: Float>(t1234: F, t24680: F, t1222: F, t140: F, t24826: F, t1209: F, t24864: F, t473: F, t24704: F, t3153: F, t13045: F, t6622: F) -> (F, F, F, F, F, F) {
    let t84185 = t1234 * t24680;
    let t84195 = t1222 * t140 * t24826;
    let t84315 = t1209 * t24864;
    let t84429 = t473 * t24864;
    let t84487 = t24704 * t3153;
    let t84636 = t13045 * t6622;
    (t84185, t84195, t84315, t84429, t84487, t84636)
}
