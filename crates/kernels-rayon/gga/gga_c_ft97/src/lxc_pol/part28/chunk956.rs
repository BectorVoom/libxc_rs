//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 956/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk956(t32063: f64, t32078: f64, t7238: f64, t1317: f64, t1637: f64, t7248: f64, t1636: f64, t7256: f64, t89: f64, t7260: f64, t32360: f64, t375: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t137180 = t7238 * t32063 * t32078;
    let t137197 = t1317 * t1637 * t7248;
    let t137198 = 4.0_f64 / 27.0_f64 * t137197;
    let t137204 = t89 * t1636 * t7256;
    let t137205 = 8.0_f64 / 27.0_f64 * t137204;
    let t137212 = t89 * t1636 * t7260;
    let t137213 = 4.0_f64 / 27.0_f64 * t137212;
    let t137215 = t89 * t375 * t32360;
    (t137180, t137197, t137198, t137204, t137205, t137212, t137213, t137215)
}
