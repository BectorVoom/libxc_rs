//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1292/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1292(t11326: f64, t15108: f64, t15111: f64, t15114: f64, t15116: f64, t15119: f64, t15121: f64, t15123: f64, t15127: f64, t15132: f64, t15178: f64, t15181: f64, t15184: f64, t15187: f64, t15189: f64, t15195: f64, t15200: f64, t15435: f64, t15450: f64, t15457: f64, t15459: f64, t15472: f64) -> f64 {
    let t15474 = 0.142419375e1_f64 * t15108 - 0.76790625e-1_f64 * t15111 - 0.1898925e1_f64 * t15114 - 0.9494625e0_f64 * t15116 + 0.3071625e0_f64 * t15119 + 0.15358125e0_f64 * t15121 - 0.91285185185185185185e-1_f64 * t15123 - t15435 + 0.13287407407407407408e0_f64 * t15127 - 0.39862222222222222222e0_f64 * t15132 + t15450 - 0.27385555555555555556e-1_f64 * t15178 - 0.36514074074074074075e-1_f64 * t15181 + 0.32862666666666666666e0_f64 * t15184 + 0.16431333333333333333e0_f64 * t15187 - 0.13287407407407407408e0_f64 * t15189 + t15457 - 0.29896666666666666667e0_f64 * t15195 + t15459 - 0.82156666666666666667e-1_f64 * t15200 - 0.10954222222222222222e0_f64 * t11326 + t15472;
    t15474
}
