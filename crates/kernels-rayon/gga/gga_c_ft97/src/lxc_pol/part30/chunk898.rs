//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 898/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk898(t1609: f64, t1610: f64, t1613: f64, t19: f64, t7: f64, t11: f64, t1690: f64, t10051: f64, t754: f64, t10050: f64, t257: f64, t255: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t37481 = t1613 * t1610 * t1609;
    let t37991 = t7 * t19;
    let t38176 = t1690 * t11;
    let t41402 = t754 * t10051;
    let t41408 = 1.0_f64 / t10050 / t257;
    let t41409 = t255 * t41408;
    (t37481, t37991, t38176, t41402, t41408, t41409)
}
