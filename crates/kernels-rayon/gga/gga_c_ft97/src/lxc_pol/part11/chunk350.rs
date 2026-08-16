//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 350/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk350(t1557: f64, t2: f64, t1559: f64, t1780: f64, t355: f64, t369: f64) -> (f64, f64, f64, f64) {
    let t1781 = t2 * t1557;
    let t1782 = t1781 * t1559;
    let t1783 = t1780 * t1782;
    let t1786 = t355 * t369;
    (t1781, t1782, t1783, t1786)
}
