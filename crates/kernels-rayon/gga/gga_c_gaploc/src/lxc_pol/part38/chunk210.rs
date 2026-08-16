//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 210/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk210(t1029: f64, t738: f64, t1025: f64, t270: f64, t946: f64, t1022: f64, t314: f64) -> (f64, f64, f64) {
    let t1030 = t738 * t1029;
    let t1033 = 0.76905262301422242837e-2_f64 * t270 * t1025 + 0.64087718584518535698e-3_f64 * t946 - 0.76905262301422242837e-2_f64 * t270 * t1030;
    let t1035 = t314 * t1022;
    (t1030, t1033, t1035)
}
