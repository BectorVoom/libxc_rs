//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 582/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk582(t419: f64, t8106: f64, t424: f64, t626: f64, t173: f64, t1747: f64, t1738: f64, t1570: f64, t23: f64, t7763: f64, t7765: f64, t420: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8107 = t419 * t8106;
    let t8109 = t626 * t424;
    let t8110 = t419 * t8109;
    let t8112 = t173 * t1747;
    let t8113 = t419 * t8112;
    let t8115 = t173 * t1738;
    let t8116 = t419 * t8115;
    let t8119 = 1.0_f64 / t23 / t1570;
    let t8120 = t8119 * t7763;
    let t8121 = t8120 * t7765;
    let t8122 = t420 * t8121;
    (t8107, t8109, t8110, t8112, t8113, t8115, t8116, t8119, t8121, t8122)
}
