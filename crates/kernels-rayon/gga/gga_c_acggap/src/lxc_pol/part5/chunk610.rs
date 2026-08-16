//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 610/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk610(t868: f64, t872: f64, t463: f64, t879: f64, t449: f64, t316: f64, t180: f64, t848: f64) -> (f64, f64, f64, f64, f64) {
    let t3886 = t868 * t872;
    let t3888 = t879 * t463;
    let t3889 = t449 * t3888;
    let t3890 = t316 * t3889;
    let t3892 = t848 * t180;
    (t3886, t3888, t3889, t3890, t3892)
}
