//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 227/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk227<F: Float>(t249: F, t433: F, t945: F, t12: F, t13: F, t140: F, t141: F, t6: F, t36: F, t214: F, t243: F, t242: F, t7: F, t5: F, t368: F, t142: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t946 = t249 * t433;
    let t948 = 0.10843581300301739842e-1 * t945 * t946;
    let t951 = 1.0 / t13 / t12 * t140;
    let t952 = t141 * t6;
    let t953 = t952 * t36;
    let t954 = t951 * t953;
    let t956 = t243 * t214;
    let t957 = t242 * t956;
    let t959 = t7 * t214;
    let t960 = t5 * t959;
    let t962 = 1.0/f64::sqrt(t12);
    let t963 = t962 * t140;
    let t964 = t963 * t953;
    let t966 = t368 * t956;
    let t969 = t142 * t6 * t36;
    (t946, t948, t951, t952, t954, t957, t959, t960, t963, t964, t966, t969)
}
