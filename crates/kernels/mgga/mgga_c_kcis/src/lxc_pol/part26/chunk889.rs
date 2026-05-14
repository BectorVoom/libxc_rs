//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 889/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk889<F: Float>(t1464: F, t22240: F, t3734: F, t7258: F, t1014: F, t7105: F, t7108: F, t1489: F, t7257: F, t1396: F, t4123: F, t15955: F, t5671: F, t12241: F, t15909: F, t3728: F, t6919: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22241 = t1464 * t22240;
    let t22243 = t3734 * t7258;
    let t22244 = t1464 * t22243;
    let t22248 = t1014 * t7105;
    let t22250 = t1014 * t7108;
    let t22252 = t7257 * t1489;
    let t22253 = t1396 * t22252;
    let t22254 = t4123 * t22253;
    let t22255 = t1464 * t22254;
    let t22259 = t15955 * t5671;
    let t22260 = t12241 * t22259;
    let t22261 = t15909 * t22260;
    let t22263 = t3728 * t6919;
    (t22241, t22244, t22248, t22250, t22252, t22255, t22259, t22261, t22263)
}
