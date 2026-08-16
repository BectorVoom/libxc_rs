//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 588/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk588(t1386: f64, t8232: f64, t1882: f64, t5953: f64, t376: f64, t5931: f64, t89: f64, t23898: f64, t23923: f64, t1380: f64, t1637: f64, t5780: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24003 = 4.0_f64 / 27.0_f64 * t8232 * t1386;
    let t24004 = t1882 * t5953;
    let t24007 = t89 * t376 * t5931;
    let t24034 = 2.0_f64 / 27.0_f64 * t23898;
    let t24041 = 4.0_f64 / 27.0_f64 * t23923;
    let t24054 = 4.0_f64 / 27.0_f64 * t89 * t1637 * t1380;
    let t24073 = t376 * t5780;
    (t24003, t24004, t24007, t24034, t24041, t24054, t24073)
}
