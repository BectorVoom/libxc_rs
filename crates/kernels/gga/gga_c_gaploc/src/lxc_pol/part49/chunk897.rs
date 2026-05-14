//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 897/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk897<F: Float>(t43907: F, t41330: F, t41337: F, t41340: F, t13077: F, t28439: F, t32744: F, t9824: F, t10924: F, t1980: F, t13065: F, t2013: F, t43710: F, t825: F, t969: F, t41342: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t43908 = 0.17875244975925213335e0 * t43907;
    let t43909 = 0.11502877786176224903e1 * t41330;
    let t43910 = 0.11916829983950142223e0 * t41337;
    let t43911 = 0.89376224879626066674e-1 * t41340;
    let t43912 = t13077 * t28439;
    let t43913 = 0.59584149919750711116e-1 * t43912;
    let t43914 = t32744 * t9824;
    let t43915 = 0.29792074959875355558e-1 * t43914;
    let t43917 = t1980 * t10924 * t9824;
    let t43918 = 0.29792074959875355558e-1 * t43917;
    let t43919 = t2013 * t13065;
    let t43922 = t825 * t969 * t43710;
    let t43924 = 0.29792074959875355558e-1 * t41342;
    (t43908, t43909, t43910, t43911, t43913, t43915, t43918, t43919, t43922, t43924)
}
