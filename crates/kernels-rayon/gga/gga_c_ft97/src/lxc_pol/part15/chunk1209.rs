//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1209/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1209(t72080: f64, t72082: f64, t83619: f64, t83652: f64, t83655: f64, t83683: f64, t89824: f64, t89828: f64, t89834: f64, t89837: f64, t89840: f64, t89845: f64, t89851: f64, t89855: f64) -> f64 {
    let t91171 = 4.0_f64 / 9.0_f64 * t83619 - 2.0_f64 / 3.0_f64 * t89824 + 2.0_f64 / 9.0_f64 * t89828 - 4.0_f64 / 3.0_f64 * t83652 + 4.0_f64 / 9.0_f64 * t83655 + t89834 / 3.0_f64 + 4.0_f64 / 3.0_f64 * t89837 - 4.0_f64 / 27.0_f64 * t89840 - 8.0_f64 / 27.0_f64 * t83683 - 40.0_f64 / 243.0_f64 * t89845 + t72080 + t72082 + 4.0_f64 / 3.0_f64 * t89851 + t89855 / 3.0_f64;
    t91171
}
