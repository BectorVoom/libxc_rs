//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2733/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2733(t5672: f64, t828: f64, t4363: f64, t2565: f64, t702: f64, t9305: f64) -> (f64, f64, f64) {
    let t36776 = t5672 * t828;
    let t36833 = t4363 * t828;
    let t39419 = 8.0_f64 * t2565 * t702 * t9305;
    (t36776, t36833, t39419)
}
