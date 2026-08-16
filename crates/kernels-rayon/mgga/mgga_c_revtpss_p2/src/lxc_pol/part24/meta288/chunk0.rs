//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1069/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1069(t19976: f64, t3115: f64, t4817: f64, t4834: f64, t127: f64, t371: f64, t6337: f64, t3205: f64, t6276: f64, t1025: f64, t4845: f64, t4858: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19977 = t3115 * t19976;
    let t20005 = t4834 * t4817;
    let t20016 = t371 * t127 * t6337;
    let t20017 = t3205 * t20016;
    let t20020 = t371 * t127 * t6276;
    let t20021 = t1025 * t20020;
    let t20025 = t4858 * t4845;
    (t19977, t20005, t20016, t20017, t20020, t20021, t20025)
}
