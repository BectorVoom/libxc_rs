//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1547/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1547<F: Float>(t1160: F, t24453: F, t24362: F, t3479: F, t24407: F, t3523: F, t1179: F, t24252: F, t24864: F, t460: F, t5219: F, t6695: F) -> (F, F, F, F, F, F) {
    let t81791 = t24453 * t1160;
    let t81836 = t24362 * t3479;
    let t81873 = t24407 * t3523;
    let t82050 = t24252 * t1179;
    let t82147 = t460 * t24864;
    let t82150 = t5219 * t6695;
    (t81791, t81836, t81873, t82050, t82147, t82150)
}
