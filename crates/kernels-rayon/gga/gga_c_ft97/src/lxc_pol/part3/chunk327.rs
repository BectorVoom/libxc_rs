//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 327/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk327(t397: f64, t53: f64, t407: f64, t76: f64, t66: f64, t172: f64, t391: f64, t68: f64, t72: f64, t67: f64, t9: f64) -> (f64, f64, f64, f64, f64) {
    let t1702 = t53 * t397;
    let t1710 = 1.0_f64 / t407 / t76;
    let t1711 = t66 * t1710;
    let t1720 = t391 * t172;
    let t1722 = t68 * t1720 * t72;
    let t1725 = t9 * t67 * t391;
    (t1702, t1710, t1711, t1722, t1725)
}
