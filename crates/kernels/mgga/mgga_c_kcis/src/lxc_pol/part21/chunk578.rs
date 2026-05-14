//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 578/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk578<F: Float>(t1008: F, t4781: F, t1014: F, t1750: F, t1126: F, t1749: F, t303: F, t1800: F, t922: F) -> (F, F, F, F, F) {
    let t4782 = t4781 * t1008;
    let t4787 = t1014 * t1750;
    let t4789 = t1749 * t1126;
    let t4790 = t303 * t4789;
    let t4792 = t1800 * t922;
    (t4782, t4787, t4789, t4790, t4792)
}
