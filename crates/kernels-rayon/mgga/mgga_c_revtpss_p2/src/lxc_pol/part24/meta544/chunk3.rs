//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1609/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1609(t6048: f64, t40998: f64, t41003: f64, t41037: f64, t41049: f64, t41078: f64, t50248: f64, t51203: f64, t51211: f64, t61448: f64, t62528: f64, t76020: f64, t76026: f64, t76051: f64, t76058: f64, t76062: f64, t865: f64) -> f64 {
    let t87361 = t6048 * t6048;
    let t87373 = 0.65854491829355115985e-1_f64 * t76020 - 0.13170898365871023197e0_f64 * t76026 + 0.15805078039045227836e2_f64 * t865 * t41078 * t87361 + 0.44178176337912614788e-3_f64 * t50248 - t40998 - t41003 + 0.43902994552903410657e-1_f64 * t61448 + t41037 - 0.1561190486301245283e0_f64 * t62528 + 0.65854491829355115985e-1_f64 * t76051 + t41049 + 0.18505311230957427423e-1_f64 * t51203 - 0.39029762157531132076e-1_f64 * t76058 + 0.23417857294518679246e0_f64 * t76062 + 0.12142592671231907757e0_f64 * t51211;
    t87373
}
