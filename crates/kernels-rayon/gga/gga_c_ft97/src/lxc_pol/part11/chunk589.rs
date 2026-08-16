//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 589/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk589(t370: f64, t8183: f64, t27: f64, t89: f64, t10: f64, t3050: f64, t83: f64, t1636: f64, t433: f64, t1756: f64, t375: f64, t7804: f64, t7809: f64, t7813: f64, t7817: f64, t7820: f64, t7822: f64, t7827: f64, t7831: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8184 = t370 * t8183;
    let t8186 = t89 * t27 * t8184;
    let t8189 = t10 * t3050 * t83;
    let t8190 = 14.0_f64 / 81.0_f64 * t8189;
    let t8192 = t89 * t1636 * t433;
    let t8195 = t89 * t375 * t1756;
    let t8197 = 2.0_f64 / 9.0_f64 * t7804 - t7809 / 9.0_f64 + t7813 / 6.0_f64 + t7817 / 6.0_f64 - t7820 / 9.0_f64 - t7822 / 9.0_f64 - t7827 / 3.0_f64 - t7831 / 3.0_f64 - t8186 / 6.0_f64 - t8190 - 2.0_f64 / 9.0_f64 * t8192 + t8195 / 6.0_f64;
    (t8184, t8186, t8189, t8192, t8195, t8197)
}
