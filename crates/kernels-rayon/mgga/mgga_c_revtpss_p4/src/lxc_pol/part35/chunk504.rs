//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 504/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk504(t1626: f64, t964: f64, t1633: f64, t3014: f64, t300: f64, t2986: f64, t1646: f64, t993: f64) -> (f64, f64, f64, f64, f64) {
    let t4685 = t1626 * t964;
    let t4711 = t1633 * t3014;
    let t4719 = t300 * t1626;
    let t4724 = t2986 * t1633;
    let t4746 = t1646 * t993;
    (t4685, t4711, t4719, t4724, t4746)
}
