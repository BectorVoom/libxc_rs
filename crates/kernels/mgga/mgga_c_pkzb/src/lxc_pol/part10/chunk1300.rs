//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1300/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1300<F: Float>(t25714: F, t665: F, t1873: F, t25704: F, t2759: F, t667: F, t7375: F, t7378: F, t17351: F, t17354: F, t17405: F, t17411: F, t17566: F, t20705: F, t25705: F, t1878: F, t218: F, t3542: F) -> (F, F, F, F, F, F) {
    let t25715 = t665 * t25714;
    let t25717 = t1873 * t25704;
    let t25722 = t667 * t2759;
    let t25723 = t7375 * t25722;
    let t25725 = t7378 * t25722;
    let t25729 = -0.1898925e1 * t25705 + 0.1898925e1 * t25715 + 0.3071625e0 * t25717 - 0.1460562962962962963e1 * t17405 + 0.27385555555555555556e0 * t17411 - 0.1860237037037037037e1 * t20705 + 0.5696775e1 * t25723 - 0.3071625e0 * t25725 + t17566 - 0.18602370370370370371e1 * t17351 + 0.39862222222222222223e0 * t17354;
    let t25734 = t218 * t1878 * t3542;
    (t25715, t25717, t25723, t25725, t25729, t25734)
}
