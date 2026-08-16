//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3743/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3743(t43865: f64, t43888: f64, t43890: f64, t43892: f64, t56230: f64, t56236: f64, t68389: f64, t68393: f64, t68397: f64, t68399: f64, t68454: f64, t68456: f64, t68459: f64) -> f64 {
    let t71176 = -0.55555555555555555556e-2_f64 * t56230 - 0.17283950617283950617e-1_f64 * t56236 - 0.55555555555555555556e-2_f64 * t68389 + 0.83333333333333333333e-2_f64 * t68393 - 0.11111111111111111111e-1_f64 * t68397 + 0.74074074074074074076e-2_f64 * t68399 - 0.24691358024691358025e-2_f64 * t43865 - 0.17283950617283950617e-1_f64 * t43888 + 0.37037037037037037037e-2_f64 * t43890 + 0.74074074074074074074e-2_f64 * t43892 - 0.22222222222222222222e-1_f64 * t68454 - 0.33333333333333333334e-1_f64 * t68456 + 0.50000000000000000001e-1_f64 * t68459;
    t71176
}
