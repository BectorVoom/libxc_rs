//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 905/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk905<F: Float>(t33953: F, t5150: F, t13364: F, t33952: F, t154: F, t506: F, t7322: F, t7326: F, t7315: F, t8589: F, t2046: F, t336: F, t4099: F, t579: F, t30226: F, t30240: F) -> (F, F, F, F, F, F, F) {
    let t33954 = t33953 * t5150;
    let t33956 = t33952 * t13364 * t33954;
    let t33960 = t7322 * t154 * t506 * t7326;
    let t33962 = t7315 * t8589;
    let t33963 = 11.0 / 192.0 * t33962;
    let t33966 = t2046 * t336 * t579 * t4099;
    let t33968 = 0.17149607247227894789e-2 * t30226;
    let t33970 = 0.21437009059034868486e-3 * t30240;
    (t33954, t33956, t33960, t33963, t33966, t33968, t33970)
}
