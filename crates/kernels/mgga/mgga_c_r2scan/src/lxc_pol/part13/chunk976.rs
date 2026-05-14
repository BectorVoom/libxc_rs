//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 976/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk976<F: Float>(t10848: F, t10903: F, t2207: F, t10698: F, t10716: F, t10810: F, t1577: F, t6536: F, t2201: F, t2252: F, t3319: F, t3320: F, t1234: F, t505: F, t6159: F, t6162: F) -> (F, F, F, F, F, F) {
    let t38099 = t2207 * t10903 * t10848;
    let t38111 = t10698 * t10716;
    let t38114 = t1577 * t10810 * t6536;
    let t38123 = t2201 * t3319 * t3320 * t2252;
    let t38127 = t2207 * t3319 * t3320 * t1234;
    let t38130 = t6159 * t505 * t6162;
    (t38099, t38111, t38114, t38123, t38127, t38130)
}
