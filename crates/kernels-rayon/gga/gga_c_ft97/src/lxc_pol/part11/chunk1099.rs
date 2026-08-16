//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1099/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1099(t245: f64, t10174: f64, t1580: f64, t21: f64, t2624: f64, t267: f64, t363: f64, t37391: f64, t41988: f64, t43018: f64, t5: f64, t7745: f64, t776: f64) -> f64 {
    let t246 = 10000000.0_f64 <= t245;
    let t43034 = piecewise3(t246, 0.0_f64, t5 * (t41988 + t43018) * t21 / 4.0_f64 + t5 * t10174 * t363 + 3.0_f64 / 2.0_f64 * t5 * t2624 * t1580 + t5 * t776 * t7745 + t5 * t267 * t37391 / 4.0_f64);
    t43034
}
