//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1262/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1262(t2289: f64, t2769: f64, t39097: f64, t10564: f64, t123: f64) -> (f64, f64, f64) {
    let t41687 = 1.0_f64 / t2769 / t2289;
    let t41688 = t41687 * t39097;
    let t41690 = t123 * t10564 * t41688;
    (t41687, t41688, t41690)
}
