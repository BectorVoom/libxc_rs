//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 906/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk906(t1554: f64, t1984: f64, t597: f64, t9438: f64, t605: f64, t9132: f64, t24: f64, t32905: f64, t2101: f64, t2179: f64, t2142: f64, t11119: f64, t37940: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40465 = t1554 * t1984;
    let t40591 = t597 * t9438;
    let t40792 = t9132 * t605;
    let t40830 = t24 * t32905;
    let t40911 = t2101 * t2179;
    let t40945 = t2101 * t2142;
    let t41209 = t9132 * t597;
    let t44965 = t11119 * t37940;
    (t40465, t40591, t40792, t40830, t40911, t40945, t41209, t44965)
}
