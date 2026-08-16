//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2868/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2868(t40271: f64, t40294: f64, t4514: f64, t51507: f64, t62777: f64, t62809: f64, t76127: f64, t76136: f64, t77171: f64, t77177: f64, t77183: f64, t77191: f64, t837: f64) -> f64 {
    let t77193 = 0.16463622957338778997e-1_f64 * t77171 + 0.43902994552903410656e-1_f64 * t62777 - 0.26019841438354088051e-2_f64 * t40271 - 0.32927245914677557992e-1_f64 * t77177 - t40294 - 0.65854491829355115987e0_f64 * t4514 * t76127 * t837 - 0.29272321618148349057e-1_f64 * t62809 - 0.29272321618148349057e-1_f64 * t77183 - 0.19756347548806534796e1_f64 * t4514 * t76136 * t837 + 0.43902994552903410658e-1_f64 * t51507 + 0.32927245914677557992e-1_f64 * t77191;
    t77193
}
