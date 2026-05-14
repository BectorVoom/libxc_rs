//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 836/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk836<F: Float>(t14797: F, t507: F, t4462: F, t515: F, t1524: F, t4435: F, t1197: F, t3696: F, t12974: F, t1588: F, t3532: F, t12829: F, t539: F, t1568: F, t4416: F, t12261: F, t1592: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14798 = t507 * t14797;
    let t14800 = 1.0 / t4462 / t515;
    let t14804 = t1524 * t4435;
    let t14810 = t1197 * t3696;
    let t14831 = 0.53272592592592592592e-1 * t12974;
    let t14909 = t1588 * t3532;
    let t14935 = t539 * t12829;
    let t14940 = t1568 * t4416;
    let t14942 = t12261 * t1592;
    (t14798, t14800, t14804, t14810, t14831, t14909, t14935, t14940, t14942)
}
