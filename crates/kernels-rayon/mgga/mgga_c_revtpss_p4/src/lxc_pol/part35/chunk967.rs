//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 967/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk967(t12168: f64, t24078: f64, t1651: f64, t19556: f64, t1089: f64, t1678: f64, t6299: f64, t23820: f64, t378: f64, t6305: f64, t3304: f64, t1668: f64, t6343: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24093 = t24078 * t12168;
    let t24098 = t19556 * t1651;
    let t24104 = t1678 * t6299 * t1089;
    let t24108 = t378 * t23820 * t1089;
    let t24111 = t1678 * t6305;
    let t24112 = t24111 * t3304;
    let t24116 = t6343 * t1668 * t1089;
    (t24093, t24098, t24104, t24108, t24111, t24112, t24116)
}
