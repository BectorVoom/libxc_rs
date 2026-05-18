//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1078/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1078<F: Float>(t10856: F, t5174: F, t2111: F, t2164: F, t22766: F, t20450: F, t2215: F, t10710: F, t10768: F, t20437: F, t10734: F, t571: F, t572: F) -> (F, F, F, F, F) {
    let t37998 = t10856 * t5174;
    let t38001 = t2111 * t22766 * t2164;
    let t38002 = F::new(0.1590300183910403919e-2) * t38001;
    let t38003 = t20450 * t2215;
    let t38028 = t10768 * t10710 * t20437;
    let t38031 = t571 * t572 * t10734;
    (t37998, t38002, t38003, t38028, t38031)
}
