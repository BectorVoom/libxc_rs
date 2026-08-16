//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 465/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk465(t4733: f64, t574: f64, t605: f64, t1060: f64, t569: f64, t925: f64, t167: f64, t4462: f64, t2205: f64, t4454: f64, t1039: f64, t2086: f64, t91: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4735 = t574 * t605 * t4733;
    let t4739 = t569 * t1060 * t925;
    let t4743 = t569 * t167 * t4462;
    let t4747 = t2205 * t167 * t4454;
    let t4753 = t1039 * t1039;
    let t4755 = t91 * t2086 * t4753;
    (t4735, t4739, t4743, t4747, t4753, t4755)
}
