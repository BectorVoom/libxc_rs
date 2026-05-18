//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1060/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1060<F: Float>(t167: F, t7704: F, t14554: F, t1003: F, t4781: F, t26686: F, t4977: F, t7691: F, t5329: F, t4972: F, t7709: F, t1094: F, t1748: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27903 = t7704 * t167;
    let t27904 = t14554 * t27903;
    let t27910 = t4781 * t1003;
    let t27911 = t26686 * t27910;
    let t27914 = t7691 * t4977;
    let t27915 = t5329 * t27914;
    let t27918 = t7709 * t4972;
    let t27919 = t5329 * t27918;
    let t27924 = t1748 * t1094;
    (t27903, t27904, t27910, t27911, t27914, t27915, t27918, t27919, t27924)
}
