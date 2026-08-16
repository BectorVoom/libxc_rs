//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1184/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1184(t15065: f64, t5177: f64, t284: f64, t5082: f64, t3339: f64, t1800: f64, t3361: f64, t1170: f64, t3477: f64, t5096: f64, t3432: f64, t5172: f64) -> (f64, f64, f64, f64, f64) {
    let t15066 = t15065 * t5177;
    let t15068 = t5082 * t284;
    let t15069 = t15068 * t3339;
    let t15071 = t3361 * t1800;
    let t15072 = t1170 * t15071;
    let t15074 = t3477 * t5096;
    let t15076 = t5172 * t3432;
    (t15066, t15069, t15072, t15074, t15076)
}
