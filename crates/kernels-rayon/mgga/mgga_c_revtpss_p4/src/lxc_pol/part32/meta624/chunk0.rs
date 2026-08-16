//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1968/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1968(t1448: f64, t6922: f64, t7897: f64, t8995: f64, t101448: f64, t101451: f64, t101755: f64, t101756: f64, t105870: f64, t105873: f64, t105876: f64, t105878: f64, t105881: f64, t105883: f64, t95397: f64) -> (f64, f64, f64) {
    let t109263 = t6922 * t1448;
    let t109269 = t7897 * t8995;
    let t109367 = -t95397 - t101448 - 44.0_f64 / 9.0_f64 * t101451 - t101755 + t101756 - 4.0_f64 / 3.0_f64 * t105870 - 3.0_f64 / 2.0_f64 * t105873 + t105876 + 2.0_f64 / 3.0_f64 * t105878 + t105881 / 2.0_f64 - t105883 / 4.0_f64;
    (t109263, t109269, t109367)
}
