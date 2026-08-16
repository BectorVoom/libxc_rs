//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 342/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk342(t1760: f64, t348: f64, t95: f64, t96: f64, t473: f64, t91: f64, t1542: f64, t9: f64) -> (f64, f64, f64, f64, f64) {
    let t1761 = t348 * t1760;
    let t1766 = 1.0_f64 / t96 / t95;
    let t1767 = t473 * t473;
    let t1769 = t91 * t1766 * t1767;
    let t1771 = t9 * t1542;
    (t1761, t1766, t1767, t1769, t1771)
}
