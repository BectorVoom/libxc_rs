//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1264/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1264<F: Float>(t10119: F, t1819: F, t555: F, t10123: F, t8185: F, t10137: F, t10131: F, t1782: F, t1787: F, t1796: F, t1804: F, t1807: F, t1808: F, t19698: F, t19700: F, t20006: F, t23030: F, t27035: F, t27038: F, t27066: F, t27071: F, t27085: F, t3804: F, t545: F, t558: F, t6164: F, t6190: F, t6195: F, t7835: F, t7842: F, t9909: F) -> F {
    let t27088 = t555 * t1819 * t10119;
    let t27091 = t555 * t8185 * t10123;
    let t27094 = t555 * t1819 * t10137;
    let t27096 = t27035 / F::cast_from(48.0_f64) + t7842 * t7835 * t27038 / F::cast_from(2.0_f64) - t23030 / F::cast_from(16.0_f64) + t19698 / F::cast_from(96.0_f64) + t19700 / F::cast_from(48.0_f64) + t20006 / F::cast_from(96.0_f64) - t555 * t558 * t6190 * t3804 / F::cast_from(64.0_f64) - t555 * t558 * t6195 * t3804 / F::cast_from(32.0_f64) - t555 * t558 * t1782 * t9909 / F::cast_from(32.0_f64) - t555 * t558 * t6164 * t3804 / F::cast_from(64.0_f64) - t555 * t558 * t1787 * t9909 / F::cast_from(32.0_f64) - t555 * t558 * t27066 * t545 / F::cast_from(32.0_f64) - t555 * t558 * t27071 * t545 / F::cast_from(32.0_f64) - t555 * t558 * t10131 * t1796 / F::cast_from(64.0_f64) - t1804 * t1807 * t10131 * t1808 / F::cast_from(48.0_f64) - t27085 / F::cast_from(48.0_f64) - t27088 / F::cast_from(48.0_f64) + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t27091 - t27094 / F::cast_from(96.0_f64);
    t27096
}
