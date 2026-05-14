//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1278/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1278<F: Float>(t2670: F, t7310: F, t2609: F, t27067: F, t7457: F, t7984: F, t22709: F, t6106: F, t8756: F, t1610: F, t5095: F, t9333: F, t6395: F, t9293: F, t113: F, t27914: F) -> (F, F, F, F, F, F, F) {
    let t29861 = t2670 * t7310;
    let t29866 = t27067 * t2609;
    let t29892 = t7984 * t7457;
    let t29919 = t6106 * t22709 * t8756;
    let t29932 = t5095 * t1610 * t9333;
    let t29934 = t6395 * t9293;
    let t29936 = t27914 * t113;
    (t29861, t29866, t29892, t29919, t29932, t29934, t29936)
}
