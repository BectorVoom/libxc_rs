//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 852/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk852(t33: f64, t3841: f64, t516: f64, t6416: f64, t6792: f64, t162: f64, t6791: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t6798 = piecewise3(t34, 0.0_f64, 4.0_f64 / 9.0_f64 * t3841 * t6792 + 4.0_f64 / 3.0_f64 * t516 * t6416);
    let t6800 = (t6791 + t6798) * t162;
    t6800
}
