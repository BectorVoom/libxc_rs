//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1871/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1871(t13044: f64, t13063: f64, t1042: f64, t1032: f64, t3552: f64, t1246: f64) -> (f64, f64, f64, f64) {
    let t13064 = t13044 * t13063;
    let t13065 = t1042 * t13064;
    let t13068 = t3552 * t1032;
    let t13069 = t13068 * t1246;
    (t13064, t13065, t13068, t13069)
}
