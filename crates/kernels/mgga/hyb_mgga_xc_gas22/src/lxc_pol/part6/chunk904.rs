//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 904/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk904<F: Float>(t1787: F, t3: F, t1184: F, t555: F, t6160: F, t1782: F, t1179: F, t6164: F, t125: F, t3112: F, t545: F, t2987: F, t558: F, t5871: F, t5874: F, t5876: F, t5880: F, t5881: F, t5883: F, t5886: F, t5889: F, t5904: F) -> (F, F, F, F, F, F, F) {
    let t7898 = t1787 * t3;
    let t7903 = t555 * t6160 * t1184;
    let t7905 = t1782 * t3;
    let t7909 = t6164 * t1179;
    let t7913 = t3112 * t125;
    let t7914 = t7913 * t545;
    let t7918 = -t5871 / F::new(32.0) - t5874 / F::new(64.0) + t5876 / F::new(48.0) - t5880 - t5881 / F::new(32.0) - t5883 / F::new(32.0) + t5886 / F::new(48.0) + t5889 / F::new(48.0) - t5904 / F::new(64.0) - t555 * t2987 * t7898 / F::new(16.0) + t7903 / F::new(288.0) - t555 * t2987 * t7905 / F::new(16.0) - t555 * t558 * t7909 / F::new(64.0) - t555 * t558 * t7914 / F::new(32.0);
    (t7898, t7903, t7905, t7909, t7913, t7914, t7918)
}
