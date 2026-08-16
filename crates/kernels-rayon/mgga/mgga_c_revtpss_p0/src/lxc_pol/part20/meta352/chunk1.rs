//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1283/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1283(t39436: f64, t10578: f64, t9863: f64, t762: f64, t9291: f64, t2629: f64, t2251: f64) -> (f64, f64, f64, f64, f64) {
    let t39437 = 0.65061487801810439052e-1_f64 * t39436;
    let t39438 = t10578 * t9863;
    let t39439 = 0.65061487801810439052e-1_f64 * t39438;
    let t39440 = t9291 * t762;
    let t39442 = 0.67471172535210825684e-1_f64 * t2629 * t39440;
    let t39443 = t2251 * t2251;
    (t39437, t39439, t39440, t39442, t39443)
}
