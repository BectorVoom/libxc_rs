//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 588/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk588(t241: f64, t258: f64, t3951: f64, t1162: f64, t681: f64, t89: f64, t2338: f64, t2341: f64, t2518: f64, t3688: f64, t3693: f64, t3697: f64, t3702: f64, t3707: f64, t3710: f64, t3715: f64, t3720: f64, t3824: f64, t3904: f64, t3940: f64) -> (f64, f64, f64) {
    let t3953 = t241 * t3951 * t258;
    let t3958 = t89 * t681 * t1162;
    let t3972 = -t3904 / 4.0_f64 + t3940 / 2.0_f64 + t2518 + t2338 / 9.0_f64 + t2341 / 3.0_f64 + t3688 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t3693 + t3697 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t3702 + 2.0_f64 / 3.0_f64 * t3707 + t3710 / 3.0_f64 + t3715 / 3.0_f64 + 2.0_f64 * t3720 - t3824;
    (t3953, t3958, t3972)
}
