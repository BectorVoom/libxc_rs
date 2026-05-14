//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1114/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1114<F: Float>(t32924: F, t32972: F, t33014: F, t33064: F, t752: F, t1907: F, t9694: F, t1957: F, t2793: F, t5217: F, t5219: F, t5339: F, t9696: F, t11691: F, t2799: F, t11694: F, t9699: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33066 = t32924 + t32972 + t33014 + t33064;
    let t33067 = t33066 * t752;
    let t33068 = t9694 * t1907;
    let t33070 = 2.0 * t33068 * t1957;
    let t33071 = t2793 * t5217;
    let t33073 = 2.0 * t33071 * t5219;
    let t33074 = t9696 * t5339;
    let t33075 = t11691 * t2799;
    let t33077 = 4.0 * t11694 * t9699;
    (t33066, t33067, t33068, t33070, t33071, t33073, t33074, t33075, t33077)
}
