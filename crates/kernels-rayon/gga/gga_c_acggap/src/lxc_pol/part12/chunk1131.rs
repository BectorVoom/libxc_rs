//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1131/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1131(t7839: f64, t8970: f64, t1165: f64, t2068: f64, t33706: f64, t604: f64, t7337: f64, t7338: f64, t8480: f64, t1181: f64, t20595: f64, t599: f64) -> (f64, f64, f64, f64) {
    let t36096 = t7839 * t8970;
    let t36100 = t2068 * t1165 * t604 * t33706;
    let t36103 = t7337 * t8480 * t7338;
    let t36107 = t7337 * t1181 * t599 * t20595;
    (t36096, t36100, t36103, t36107)
}
