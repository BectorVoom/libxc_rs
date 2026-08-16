//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 668/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk668(t2247: f64, t7565: f64, t55: f64, t60: f64, t606: f64, t6971: f64) -> (f64, f64, f64) {
    let t7566 = t2247 * t7565;
    let t7571 = t55 * t60;
    let t7574 = -5.0_f64 / 6.0_f64 * t7571 * t606 + t6971;
    (t7566, t7571, t7574)
}
