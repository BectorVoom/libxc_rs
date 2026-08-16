//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1531/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1531(t2924: f64, t6145: f64, t2930: f64, t4571: f64, t6094: f64, t6098: f64, t6102: f64, t1621: f64, t954: f64) -> (f64, f64, f64, f64) {
    let t6147 = 0.16081979498692535067e2_f64 * t2924 * t6145;
    let t6152 = t2930 + 0.11415555555555555555e-1_f64 * t4571 - 0.11415555555555555555e-1_f64 * t6094 + 0.34246666666666666666e-1_f64 * t6098 - 0.17123333333333333333e-1_f64 * t6102;
    let t6157 = t1621 * t1621;
    let t6158 = t6157 * t954;
    (t6147, t6152, t6157, t6158)
}
