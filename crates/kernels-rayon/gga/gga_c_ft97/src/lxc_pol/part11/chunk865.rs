//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 865/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk865(t37406: f64, t8119: f64, t37357: f64, t419: f64, t420: f64, t1747: f64, t626: f64, t1738: f64, t173: f64, t8125: f64, t8121: f64, t1557: f64, t23: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t37730 = t8119 * t37406;
    let t37733 = t419 * t420 * t37730 * t37357;
    let t37736 = t419 * t626 * t1747;
    let t37739 = t419 * t626 * t1738;
    let t37742 = t419 * t173 * t8125;
    let t37745 = t419 * t173 * t8121;
    let t37748 = 1.0_f64 / t23 / t1557;
    (t37733, t37736, t37739, t37742, t37745, t37748)
}
