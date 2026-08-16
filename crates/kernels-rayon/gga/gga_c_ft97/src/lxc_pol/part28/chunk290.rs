//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 290/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk290(t1903: f64, t3204: f64, t1902: f64, t379: f64, t447: f64, t986: f64, t1848: f64, t1883: f64, t1887: f64, t1888: f64, t1890: f64, t1901: f64, t28: f64, t3115: f64, t3172: f64, t3177: f64, t3184: f64, t3190: f64, t3196: f64, t3201: f64, t446: f64, t89: f64) -> (f64, f64) {
    let t3205 = t1903 * t3204;
    let t3206 = t1902 * t3205;
    let t3210 = t447 * t986 * t379;
    let t3213 = t1901 * t3115 / 9.0_f64 + t1883 / 27.0_f64 + t89 * t28 * t3172 / 3.0_f64 - t3177 / 9.0_f64 - t1848 / 9.0_f64 + t1887 + t1890 / 9.0_f64 + t1888 / 9.0_f64 + t1901 * t3184 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t3190 - 2.0_f64 / 27.0_f64 * t1901 * t3196 + t1901 * t3201 / 9.0_f64 + t1901 * t3206 / 9.0_f64 - t446 * t3210 / 9.0_f64;
    (t3205, t3213)
}
