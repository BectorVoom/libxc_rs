//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3302/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3302(t2782: f64, t4086: f64, t543: f64, t86441: f64, t22253: f64, t47450: f64, t47454: f64, t47455: f64, t49426: f64, t49429: f64, t49432: f64, t5767: f64, t75298: f64, t75302: f64, t75307: f64, t820: f64) -> f64 {
    let t86654 = t2782 * t4086 * t86441 * t543;
    let t86665 = 0.16463622957338778996e-1_f64 * t75298 - 0.32927245914677557992e-1_f64 * t75302 + 0.16463622957338778997e-1_f64 * t86654 - 0.19514881078765566038e-2_f64 * t49426 + 0.16463622957338778996e-1_f64 * t75307 + 0.19514881078765566038e-2_f64 * t49429 - 0.13878983423218070567e-1_f64 * t49432 - 0.46263278077393568556e-2_f64 * t47450 + t47454 - 0.26019841438354088051e-2_f64 * t47455 - 0.19756347548806534796e1_f64 * t820 * t5767 * t22253;
    t86665
}
