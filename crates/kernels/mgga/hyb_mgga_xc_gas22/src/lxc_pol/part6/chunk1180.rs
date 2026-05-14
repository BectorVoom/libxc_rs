//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1180/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1180<F: Float>(t10131: F, t1782: F, t1787: F, t1796: F, t1804: F, t1807: F, t1808: F, t19698: F, t19700: F, t20006: F, t23030: F, t27035: F, t27038: F, t27066: F, t27071: F, t27085: F, t27088: F, t27091: F, t27094: F, t3804: F, t545: F, t555: F, t558: F, t6164: F, t6190: F, t6195: F, t7835: F, t7842: F, t9909: F) -> (F,) {
    let t27096 = t27035 / 48.0 + t7842 * t7835 * t27038 / 2.0 - t23030 / 16.0 + t19698 / 96.0 + t19700 / 48.0 + t20006 / 96.0 - t555 * t558 * t6190 * t3804 / 64.0 - t555 * t558 * t6195 * t3804 / 32.0 - t555 * t558 * t1782 * t9909 / 32.0 - t555 * t558 * t6164 * t3804 / 64.0 - t555 * t558 * t1787 * t9909 / 32.0 - t555 * t558 * t27066 * t545 / 32.0 - t555 * t558 * t27071 * t545 / 32.0 - t555 * t558 * t10131 * t1796 / 64.0 - t1804 * t1807 * t10131 * t1808 / 48.0 - t27085 / 48.0 - t27088 / 48.0 + 7.0 / 48.0 * t27091 - t27094 / 96.0;
    (t27096,)
}
