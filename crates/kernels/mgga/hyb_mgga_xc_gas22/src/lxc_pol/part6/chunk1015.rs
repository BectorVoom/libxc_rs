//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1015/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1015<F: Float>(t10082: F, t26: F, t10: F, t18: F, t9909: F, t3806: F, t551: F, t127: F, t19: F, t3919: F, t547: F, t5876: F, t5880: F, t642: F, t670: F, t7879: F, t7881: F, t7885: F, t7887: F) -> (F, F, F) {
    let t10083 = t26 * t10082;
    let t10087 = t9909 * t10 * t18;
    let t10094 = t3806 * t551;
    let t10097 = -3.0 / 64.0 * t547 * t3919 - t7879 + t7881 / 48.0 - t7885 / 16.0 + t7887 / 48.0 - 3.0 / 64.0 * t19 * t10083 - 3.0 / 64.0 * t10087 * t127 - 3.0 / 64.0 * t3806 * t642 - 3.0 / 64.0 * t3806 * t670 - t10094 / 64.0 + t5876 / 96.0 - t5880;
    (t10083, t10087, t10097)
}
