//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 532/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk532(t3141: f64, t3160: f64, t19: f64, t2066: f64, t124: f64, t1149: f64, t329: f64, t107: f64, t2607: f64, t2690: f64, t4: f64, t118: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3588 = 0.38033333333333333333e1_f64 * t3141;
    let t3592 = 0.12225e1_f64 * t3160;
    let t3615 = t2066 * t19;
    let t3616 = t124 * t3615;
    let t3621 = t329 * t1149;
    let t3644 = -0.12962962962962962963e0_f64 * t4 * t2607 * t107 - 0.40124259259259259261e-1_f64 * t2690;
    let t3645 = t3644 * t118;
    (t3588, t3592, t3616, t3621, t3644, t3645)
}
