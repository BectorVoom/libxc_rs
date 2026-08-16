//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2766/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2766(t4302: f64, t9586: f64, t13312: f64, t189: f64, t4401: f64, t606: f64, t14389: f64, t2258: f64, t10612: f64, t4311: f64, t14330: f64, t14369: f64, t2251: f64) -> (f64, f64, f64, f64, f64) {
    let t50856 = t4302 * t9586;
    let t50857 = 0.56968947174242584612e-3_f64 * t50856;
    let t50861 = 36.0_f64 * t4401 * t189 * t13312 * t606;
    let t50864 = 36.0_f64 * t4401 * t14389 * t2258;
    let t50865 = t4311 * t10612;
    let t50866 = 12.0_f64 * t50865;
    let t50868 = t14330 * t14369 * t2251;
    (t50857, t50861, t50864, t50866, t50868)
}
