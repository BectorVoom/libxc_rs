//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 806/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk806<F: Float>(t20905: F, t5875: F, t4162: F, t15909: F, t17298: F, t5645: F, t5650: F, t3717: F, t7091: F, t1385: F, t2006: F, t5871: F, t303: F, t1497: F, t7257: F, t1495: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20906 = t20905 * t5875;
    let t20907 = t4162 * t20906;
    let t20908 = t15909 * t20907;
    let t20910 = t17298 * t5645;
    let t20912 = t17298 * t5650;
    let t20916 = t7091 * t3717;
    let t20917 = t20916 * t1385;
    let t20922 = t5871 * t2006;
    let t20923 = t303 * t20922;
    let t20925 = t7257 * t1497;
    let t20926 = t1495 * t20925;
    (t20906, t20908, t20910, t20912, t20916, t20917, t20923, t20925, t20926)
}
