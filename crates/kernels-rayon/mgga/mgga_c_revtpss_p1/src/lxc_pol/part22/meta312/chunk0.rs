//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1750/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1750(t58: f64, t59: f64, t10199: f64, t2851: f64, t78: f64, t3361: f64, t81: f64, t116: f64, t2319: f64) -> (f64, f64, f64, f64, f64) {
    let t10368 = 1.0_f64 / t59 / t58;
    let t10379 = 1232.0_f64 / 27.0_f64 * t10199;
    let t10389 = 1.0_f64 / t78 / t2851;
    let t10398 = 1.0_f64 / t81 / t3361;
    let t10416 = t2319 * t116;
    (t10368, t10379, t10389, t10398, t10416)
}
