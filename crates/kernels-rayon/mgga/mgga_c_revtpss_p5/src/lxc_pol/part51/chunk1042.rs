//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1042/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1042(t120179: f64, t8520: f64, t25610: f64, t126: f64, t828: f64, t32014: f64, t32017: f64, t31948: f64, t8514: f64, t94014: f64, t31991: f64, t94121: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120190 = t8520 * t120179;
    let t120191 = t25610 * t120190;
    let t120199 = t828 * t126;
    let t120201 = t32014 * t120199 * t32017;
    let t120208 = t8514 * t94014 * t31948;
    let t120218 = t94121 * t31991;
    (t120190, t120191, t120199, t120201, t120208, t120218)
}
