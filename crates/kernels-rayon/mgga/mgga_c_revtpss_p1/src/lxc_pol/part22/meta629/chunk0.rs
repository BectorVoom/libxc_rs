//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2545/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2545(t127: f64, t371: f64, t6337: f64, t3205: f64, t6276: f64, t1025: f64, t4845: f64, t4858: f64, t3172: f64, t6307: f64, t3150: f64, t4820: f64, t4879: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20016 = t371 * t127 * t6337;
    let t20017 = t3205 * t20016;
    let t20020 = t371 * t127 * t6276;
    let t20021 = t1025 * t20020;
    let t20025 = t4858 * t4845;
    let t20029 = t3172 * t6307;
    let t20030 = t3150 * t20029;
    let t20034 = t4879 * t4820;
    (t20016, t20017, t20020, t20021, t20025, t20029, t20030, t20034)
}
