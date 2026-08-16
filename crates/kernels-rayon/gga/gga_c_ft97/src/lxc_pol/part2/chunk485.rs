//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 485/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk485(t2832: f64, t295: f64, t312: f64, t681: f64, t865: f64, t89: f64, t311: f64, t869: f64) -> (f64, f64, f64) {
    let t2834 = t295 * t2832 * t312;
    let t2839 = t89 * t681 * t865;
    let t2842 = 1.0_f64 / t869 / t311;
    (t2834, t2839, t2842)
}
