//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 676/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk676<F: Float>(t1341: F, t7906: F, t1415: F, t1411: F, t1224: F, t4013: F, t7736: F, t1225: F, t7740: F, t7744: F, t4008: F, t6020: F, t2128: F, t1254: F, t2119: F, t4037: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7907 = t1341 * t7906;
    let t7908 = t1415 * t7907;
    let t7909 = t1411 * t7908;
    let t7914 = t1224 * t4013 * t7736;
    let t7917 = t1224 * t1225 * t7740;
    let t7920 = t1224 * t1225 * t7744;
    let t7922 = t4008 + 0.11872222222222222222e-1 * t6020 - 0.11872222222222222222e-1 * t7914 + 0.35616666666666666666e-1 * t7917 - 0.17808333333333333333e-1 * t7920;
    let t7927 = t2128 * t2128;
    let t7928 = t7927 * t1254;
    let t7931 = t2119 * t2119;
    let t7932 = t4037 * t7931;
    (t7907, t7908, t7909, t7914, t7917, t7920, t7922, t7927, t7928, t7931, t7932)
}
