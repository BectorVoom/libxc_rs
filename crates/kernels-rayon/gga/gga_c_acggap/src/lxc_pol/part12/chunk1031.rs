//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1031/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1031(t129: f64, t507: f64, t7585: f64, t7587: f64, t30546: f64, t8477: f64, t1967: f64, t8561: f64, t30543: f64, t8515: f64, t10146: f64, t420: f64, t576: f64) -> (f64, f64, f64, f64, f64) {
    let t34345 = t129 * t507;
    let t34347 = t7585 * t34345 * t7587;
    let t34349 = t30546 * t8477;
    let t34351 = t1967 * t8561;
    let t34361 = t30543 * t8515;
    let t34368 = t576 * t420 * t10146;
    (t34347, t34349, t34351, t34361, t34368)
}
