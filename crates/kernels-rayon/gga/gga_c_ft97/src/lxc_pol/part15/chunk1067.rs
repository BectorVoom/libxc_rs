//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1067/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1067(t1969: f64, t446: f64, t86630: f64, t86618: f64, t9073: f64, t39693: f64, t86626: f64, t86614: f64, t9049: f64, t27: f64, t526: f64, t86868: f64, t89: f64) -> (f64, f64, f64, f64, f64) {
    let t87024 = t446 * t1969 * t86630;
    let t87027 = t446 * t9073 * t86618;
    let t87030 = t446 * t39693 * t86626;
    let t87033 = t446 * t9049 * t86614;
    let t87037 = t89 * t27 * t526 * t86868;
    (t87024, t87027, t87030, t87033, t87037)
}
