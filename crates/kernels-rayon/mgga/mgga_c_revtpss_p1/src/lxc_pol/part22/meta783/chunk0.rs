//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2873/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2873(t17847: f64, t3588: f64, t17854: f64, t1209: f64, t17887: f64, t12657: f64, t3754: f64, t12722: f64, t3555: f64, t12640: f64, t3552: f64, t3766: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45675 = t17847 * t3588;
    let t45679 = t17854 * t3588;
    let t45683 = t1209 * t17887;
    let t45697 = t12657 * t3754;
    let t45700 = t3555 * t12722;
    let t45707 = t12640 * t3754;
    let t45710 = t3552 * t3766;
    (t45675, t45679, t45683, t45697, t45700, t45707, t45710)
}
