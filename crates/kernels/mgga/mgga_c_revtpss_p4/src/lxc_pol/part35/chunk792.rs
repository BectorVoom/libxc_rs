//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 792/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk792<F: Float>(t136: F, t1903: F, t2457: F, t9674: F, t10073: F, t5737: F, t10069: F, t1892: F, t3964: F, t2435: F, t5760: F, t3999: F) -> (F, F, F, F, F, F, F) {
    let t14103 = t1903 * t136;
    let t14104 = t14103 * t2457;
    let t14105 = t9674 * t14104;
    let t14120 = t10073 * t5737;
    let t14149 = t10069 * t5737;
    let t14159 = t1892 * t136;
    let t14161 = t3964 * t14159 * t2457;
    let t14166 = t2435 * t5760;
    let t14171 = t3999 * t1892;
    (t14104, t14105, t14120, t14149, t14161, t14166, t14171)
}
