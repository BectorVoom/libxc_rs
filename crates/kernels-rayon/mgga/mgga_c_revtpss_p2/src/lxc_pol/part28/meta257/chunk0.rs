//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1145/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1145(t651: f64, t6993: f64, t112: f64, t624: f64, t655: f64, t68: f64) -> (f64, f64, f64) {
    let t6995 = 2.0_f64 * t651 * t6993;
    let t6996 = t624 * t112;
    let t6997 = t6996 / 3.0_f64;
    let t6998 = t68 * t655;
    (t6995, t6997, t6998)
}
