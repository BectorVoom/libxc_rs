//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 279/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk279(t1073: f64, t637: f64, t639: f64, t1068: f64, t629: f64, t631: f64, t184: f64) -> (f64, f64, f64) {
    let t1075 = t637 * t639 * t1073;
    let t1078 = t629 + t631 * t1068 / 6.0_f64 + t631 * t1075 / 2.0_f64;
    let t1079 = t1078 * t184;
    (t1075, t1078, t1079)
}
