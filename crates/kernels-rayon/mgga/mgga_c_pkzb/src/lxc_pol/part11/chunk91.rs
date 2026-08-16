//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 91/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk91(t218: f64, t219: f64, t220: f64, t210: f64, t213: f64, t216: f64) -> (f64, f64, f64, f64) {
    let t222 = t218 * t219 * t220;
    let t224 = 0.379785e1_f64 * t213 + 0.8969e0_f64 * t210 + 0.204775e0_f64 * t216 + 0.123235e0_f64 * t222;
    let t227 = 1.0_f64 + 0.16081979498692535067e2_f64 / t224;
    let t228 = f64::ln(t227);
    (t222, t224, t227, t228)
}
