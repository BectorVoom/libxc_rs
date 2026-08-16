//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 34/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk34(t65: f64, t68: f64, t55: f64, t58: f64, t61: f64) -> (f64, f64, f64, f64) {
    let t69 = t65 * t68;
    let t71 = 0.379785e1_f64 * t58 + 0.8969e0_f64 * t55 + 0.204775e0_f64 * t61 + 0.123235e0_f64 * t69;
    let t74 = 1.0_f64 + 0.16081979498692535067e2_f64 / t71;
    let t75 = f64::ln(t74);
    (t69, t71, t74, t75)
}
