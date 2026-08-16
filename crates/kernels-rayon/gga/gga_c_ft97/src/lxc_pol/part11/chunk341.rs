//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 341/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk341(t1711: f64, t1712: f64, t14: f64, t1675: f64, t68: f64, t72: f64, t172: f64, t391: f64, t67: f64, t9: f64) -> (f64, f64, f64, f64) {
    let t1713 = t1711 * t1712;
    let t1716 = t1675 * t14;
    let t1718 = t68 * t1716 * t72;
    let t1720 = t391 * t172;
    let t1722 = t68 * t1720 * t72;
    let t1725 = t9 * t67 * t391;
    (t1713, t1718, t1722, t1725)
}
