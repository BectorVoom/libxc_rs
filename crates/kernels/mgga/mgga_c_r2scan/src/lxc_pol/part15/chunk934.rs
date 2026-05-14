//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 934/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk934<F: Float>(t10979: F, t128: F, t1551: F, t6212: F, t409: F, t5: F, t511: F, t7: F, t2096: F, t2185: F, t4145: F, t1570: F, t146: F, t2078: F, t2145: F, t1543: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20421 = t10979 * t128;
    let t20437 = t6212 * t1551;
    let t20450 = t5 * t7 * t409 * t511;
    let t20544 = t2096 * t2096;
    let t20590 = t6212 * t2185;
    let t20621 = t4145 * t128;
    let t20665 = t6212 * t1570;
    let t20825 = t146 * t2145 * t2078;
    let t20853 = t6212 * t1543;
    (t20421, t20437, t20450, t20544, t20590, t20621, t20665, t20825, t20853)
}
