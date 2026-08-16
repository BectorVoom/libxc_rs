//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 2/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk2(t3: f64, t2: f64, param_C0_c_0: f64, param_C0_c_1: f64, param_C0_c_2: f64, param_C0_c_3: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4 = 1.0_f64 / t3;
    let t5 = t2 * t4;
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t9 = param_C0_c_0;
    let t10 = param_C0_c_1;
    let t11 = param_C0_c_2;
    let t12 = param_C0_c_3;
    let t14 = t2 * t2;
    (t4, t5, t9, t10, t11, t12, t14)
}
