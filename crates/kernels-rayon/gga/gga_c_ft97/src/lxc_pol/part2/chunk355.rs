//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 355/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk355(t1852: f64, t1853: f64, t83: f64, t379: f64, t447: f64, t499: f64, t110: f64, t1651: f64, t1642: f64, t82: f64) -> (f64, f64, f64, f64, f64) {
    let t1854 = t1852 * t1853;
    let t1855 = t83 * t1854;
    let t1859 = t447 * t499 * t379;
    let t1863 = t447 * t110 * t1651;
    let t1866 = t1642 * t82;
    (t1854, t1855, t1859, t1863, t1866)
}
