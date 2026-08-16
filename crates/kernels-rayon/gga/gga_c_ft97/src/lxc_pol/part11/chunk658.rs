//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 658/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk658(t2190: f64, t379: f64, t9144: f64, t2142: f64, t2157: f64, t144: f64, t1901: f64, t446: f64, t9090: f64, t9094: f64, t9097: f64, t9100: f64, t9104: f64, t9106: f64, t9109: f64, t9112: f64, t9118: f64, t9124: f64, t9129: f64, t9136: f64, t9141: f64) -> (f64, f64, f64, f64, f64) {
    let t9145 = t2190 * t379;
    let t9146 = t9144 * t9145;
    let t9149 = t2142 * t2157;
    let t9150 = t144 * t9149;
    let t9152 = -2.0_f64 / 9.0_f64 * t9090 + t1901 * t9094 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t9097 + 2.0_f64 / 3.0_f64 * t1901 * t9100 - t446 * t9104 + t9106 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t9109 - 2.0_f64 / 9.0_f64 * t9112 + 2.0_f64 / 9.0_f64 * t1901 * t9118 + 2.0_f64 / 9.0_f64 * t1901 * t9124 - 2.0_f64 / 3.0_f64 * t1901 * t9129 - 2.0_f64 / 3.0_f64 * t1901 * t9136 - 2.0_f64 / 3.0_f64 * t1901 * t9141 - 2.0_f64 / 3.0_f64 * t1901 * t9146 - t446 * t9150;
    (t9145, t9146, t9149, t9150, t9152)
}
