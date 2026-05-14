//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1017/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1017<F: Float>(t10131: F, t545: F, t10102: F, t10105: F, t10107: F, t10111: F, t10115: F, t10119: F, t10123: F, t10129: F, t1804: F, t1807: F, t2987: F, t555: F, t558: F, t5886: F, t5889: F, t7903: F) -> (F, F) {
    let t10132 = t10131 * t545;
    let t10136 = t5886 / 96.0 + t5889 / 96.0 - t10102 / 96.0 - t10105 / 192.0 - t1804 * t1807 * t10107 / 48.0 - t1804 * t1807 * t10111 / 48.0 - t555 * t558 * t10115 / 32.0 - t555 * t558 * t10119 / 32.0 - t555 * t2987 * t10123 / 16.0 + t7903 / 144.0 - t10129 / 144.0 - t555 * t558 * t10132 / 64.0;
    (t10132, t10136)
}
