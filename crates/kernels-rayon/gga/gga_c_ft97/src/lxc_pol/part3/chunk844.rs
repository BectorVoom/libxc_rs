//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 844/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk844(t2142: f64, t4733: f64, t574: f64, t1053: f64, t3408: f64, t605: f64, t1017: f64, t3565: f64, t1060: f64, t3052: f64, t569: f64, t4462: f64, t616: f64) -> (f64, f64, f64, f64, f64) {
    let t17115 = t574 * t2142 * t4733;
    let t17118 = t3408 * t1053;
    let t17120 = t574 * t605 * t17118;
    let t17123 = t1017 * t3565;
    let t17125 = t574 * t605 * t17123;
    let t17129 = t569 * t1060 * t3052;
    let t17133 = t569 * t616 * t4462;
    (t17115, t17120, t17125, t17129, t17133)
}
