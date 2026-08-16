//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 694/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk694(t1902: f64, t20214: f64, t4431: f64, t979: f64, t1910: f64, t1909: f64, t15978: f64, t15980: f64, t16083: f64, t16126: f64, t1901: f64, t20172: f64, t20179: f64, t20184: f64, t20188: f64, t20193: f64, t20196: f64, t20200: f64, t20205: f64, t20210: f64, t446: f64) -> (f64, f64, f64, f64, f64) {
    let t20215 = t1902 * t20214;
    let t20218 = t4431 * t979;
    let t20219 = t1910 * t20218;
    let t20220 = t1909 * t20219;
    let t20223 = t15978 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t15980 - 2.0_f64 / 3.0_f64 * t1901 * t20172 - 2.0_f64 / 9.0_f64 * t16083 - t16126 / 3.0_f64 - 2.0_f64 * t446 * t20179 + 2.0_f64 * t446 * t20184 + 2.0_f64 * t446 * t20188 + t446 * t20193 - t446 * t20196 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t446 * t20200 + 2.0_f64 / 9.0_f64 * t1901 * t20205 + 2.0_f64 / 9.0_f64 * t1901 * t20210 + t1901 * t20215 / 3.0_f64 + t1901 * t20220 / 3.0_f64;
    (t20215, t20218, t20219, t20220, t20223)
}
