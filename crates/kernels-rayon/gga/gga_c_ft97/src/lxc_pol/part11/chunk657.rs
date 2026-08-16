//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 657/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk657(t157: f64, t9132: f64, t160: f64, t1986: f64, t379: f64, t2178: f64, t2180: f64, t2210: f64, t2101: f64, t605: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9133 = t9132 * t157;
    let t9135 = t160 * t1986 * t379;
    let t9136 = t9133 * t9135;
    let t9140 = t2178 * t2180 * t379;
    let t9141 = t2210 * t9140;
    let t9144 = t2101 * t605;
    (t9133, t9135, t9136, t9140, t9141, t9144)
}
