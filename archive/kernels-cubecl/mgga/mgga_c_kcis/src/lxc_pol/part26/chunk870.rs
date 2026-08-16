//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 870/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk870<F: Float>(t3728: F, t7263: F, t7204: F, t2001: F, t4134: F, t5875: F, t4162: F, t15909: F, t17298: F, t5645: F, t5650: F, t3717: F, t7091: F) -> (F, F, F, F, F, F, F) {
    let t20900 = t3728 * t7263;
    let t20902 = t3728 * t7204;
    let t20905 = t4134 * t2001;
    let t20906 = t20905 * t5875;
    let t20907 = t4162 * t20906;
    let t20908 = t15909 * t20907;
    let t20910 = t17298 * t5645;
    let t20912 = t17298 * t5650;
    let t20916 = t7091 * t3717;
    (t20900, t20902, t20906, t20908, t20910, t20912, t20916)
}
