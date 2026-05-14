//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1261/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1261<F: Float>(t26926: F, t2823: F, t2055: F, t2056: F, t3128: F, t2049: F, t759: F, t761: F, t9005: F, t2060: F, t2062: F, t8589: F, t5998: F, t9033: F, t6001: F, t7872: F, t7902: F) -> (F, F, F, F, F, F, F, F) {
    let t28922 = t2823 * t26926;
    let t28933 = t2055 * t3128 * t2056;
    let t28976 = t759 * t3128 * t2049;
    let t28982 = t759 * t9005 * t761;
    let t28989 = t2060 * t8589 * t2062;
    let t28991 = t9033 * t5998;
    let t28993 = t9033 * t6001;
    let t28995 = t7872 * t7902;
    (t28922, t28933, t28976, t28982, t28989, t28991, t28993, t28995)
}
