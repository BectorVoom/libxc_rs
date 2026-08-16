//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1139/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1139(t13773: f64, t13814: f64, t13860: f64, t13931: f64, t13965: f64, t14002: f64, t14033: f64, t14063: f64, t225: f64, t5774: f64, t72: f64, t686: f64) -> (f64, f64, f64) {
    let t14066 = t13773 + t13814 + t13860 + t13931 + t13965 + t14002 + t14033 + t14063;
    let t14067 = t14066 * t225;
    let t14078 = t5774 * t72;
    let t14079 = t14078 * t686;
    (t14066, t14067, t14079)
}
