//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1099/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1099(t10914: f64, t21504: f64, t2365: f64, t21784: f64, t6111: f64, t10928: f64, t6574: f64, t822: f64, t123: f64, t15499: f64, t21503: f64, t883: f64) -> (f64, f64, f64, f64) {
    let t28633 = 0.17875244975925213335e0_f64 * t10914 * t2365 * t21504;
    let t28636 = 0.59584149919750711116e-1_f64 * t6111 * t2365 * t21784;
    let t28640 = t822 * t10928 * t6574;
    let t28641 = t15499 * t123;
    let t28645 = 0.46011511144704899612e1_f64 * t28640 * t28641 * t883 * t21503;
    (t28633, t28636, t28640, t28645)
}
