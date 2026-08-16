//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1050/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1050(t86: f64, t112: f64, t113: f64, t20479: f64, t20489: f64, t4628: f64, t4635: f64, t5: f64, t86541: f64, t86559: f64, t86571: f64, t989: f64, t992: f64) -> f64 {
    let t87 = 10000000.0_f64 <= t86;
    let t86576 = piecewise3(t87, 0.0_f64, t5 * (t86541 + t86559) * t113 / 4.0_f64 + t5 * t20479 * t992 + 3.0_f64 / 2.0_f64 * t5 * t4628 * t4635 + t5 * t989 * t20489 + t5 * t112 * t86571 / 4.0_f64);
    t86576
}
