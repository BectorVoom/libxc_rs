//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1060/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1060<F: Float>(t1804: F, t3815: F, t6214: F, t125: F, t3916: F, t545: F, t10102: F, t10105: F, t10107: F, t10111: F, t10115: F, t10119: F, t10123: F, t1807: F, t2987: F, t555: F, t558: F, t5886: F, t5889: F, t7903: F) -> (F, F, F) {
    let t10129 = t1804 * t6214 * t3815;
    let t10131 = t3916 * t125;
    let t10132 = t10131 * t545;
    let t10136 = t5886 / F::cast_from(96.0_f64) + t5889 / F::cast_from(96.0_f64) - t10102 / F::cast_from(96.0_f64) - t10105 / F::cast_from(192.0_f64) - t1804 * t1807 * t10107 / F::cast_from(48.0_f64) - t1804 * t1807 * t10111 / F::cast_from(48.0_f64) - t555 * t558 * t10115 / F::cast_from(32.0_f64) - t555 * t558 * t10119 / F::cast_from(32.0_f64) - t555 * t2987 * t10123 / F::cast_from(16.0_f64) + t7903 / F::cast_from(144.0_f64) - t10129 / F::cast_from(144.0_f64) - t555 * t558 * t10132 / F::cast_from(64.0_f64);
    (t10131, t10132, t10136)
}
