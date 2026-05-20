//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2202/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2202<F: Float>(t25082: F, t30122: F, t32113: F, t1448: F, t6781: F, t28196: F, t98495: F, t1353: F, t28197: F, t28167: F, t8717: F, t2014: F, t25190: F, t29494: F) -> (F, F, F, F, F) {
    let t109095 = F::new(6.0) * t25082 * t32113 * t30122;
    let t109096 = t6781 * t1448;
    let t109099 = F::new(6.0) * t28196 * t98495 * t109096;
    let t109100 = t6781 * t1353;
    let t109103 = F::new(6.0) * t25082 * t28197 * t109100;
    let t109104 = t30122 * t1353;
    let t109107 = F::new(12.0) * t28167 * t8717 * t109104;
    let t109110 = F::new(3.0) * t2014 * t25190 * t29494;
    (t109095, t109099, t109103, t109107, t109110)
}
