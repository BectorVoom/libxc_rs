//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1526/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1526(t11683: f64, t11710: f64, t3091: f64, t11671: f64, t3278: f64, t12020: f64, t3168: f64, t11245: f64, t42668: f64, t11628: f64, t42860: f64, t42866: f64) -> (f64, f64, f64, f64, f64) {
    let t42965 = t3091 * t11710 * t11683;
    let t42967 = t3278 * t11671;
    let t42970 = t12020 * t3168;
    let t42973 = t42668 * t11245;
    let t42977 = t42860 * t11628 * t42866;
    (t42965, t42967, t42970, t42973, t42977)
}
