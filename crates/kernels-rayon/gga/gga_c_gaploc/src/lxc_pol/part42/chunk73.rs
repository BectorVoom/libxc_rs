//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 73/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk73(t107: f64, t183: f64, t266: f64, t305: f64, t306: f64, t278: f64) -> (f64, f64) {
    let t312 = 0.58998125e-2_f64 * t305 * t306 - 0.21511666666666666667e-1_f64 * t107 * t183 * t266;
    let t313 = t312 * t278;
    (t312, t313)
}
