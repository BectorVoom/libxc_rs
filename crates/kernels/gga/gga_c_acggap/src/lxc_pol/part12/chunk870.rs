//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 870/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk870<F: Float>(t1674: F, t8034: F, t922: F, t694: F, t8052: F, t2236: F, t30005: F, t3054: F, t633: F, t865: F, t2245: F, t7924: F, t7987: F, t8100: F, t1264: F, t2131: F, t2147: F, t2225: F) -> (F, F, F, F, F, F, F) {
    let t32301 = t1674 * t8034 * t922;
    let t32313 = t694 * t8052;
    let t32315 = t30005 * t2236;
    let t32324 = 0.39512695097613069591e1 * t3054 * t633 * t865;
    let t32329 = t7924 * t2245;
    let t32331 = t7987 * t8100;
    let t32335 = t2131 * t2147 * t2225 * t1264;
    (t32301, t32313, t32315, t32324, t32329, t32331, t32335)
}
