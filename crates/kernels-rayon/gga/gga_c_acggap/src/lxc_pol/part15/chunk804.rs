//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 804/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk804(t7529: f64, t7531: f64, t7551: f64, t7572: f64, t7574: f64, t7590: f64, t7607: f64, t8190: f64, t8192: f64, t8193: f64, t8195: f64, t8205: f64, t8209: f64, t8754: f64, t8756: f64) -> f64 {
    let t9289 = -t8754 / 24.0_f64 - t8756 / 24.0_f64 - 0.41930789719472202758e-3_f64 * t7529 + 0.94344276868812456207e-3_f64 * t7531 + t8190 + t8192 + t8193 - 0.94344276868812456205e-2_f64 * t7551 - t8195 + t7572 + t7574 - t7590 - t8205 - t7607 + t8209;
    t9289
}
