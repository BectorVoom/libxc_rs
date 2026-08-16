//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 668/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk668(t670: f64, t89: f64, t9733: f64, t2404: f64, t675: f64, t2371: f64, t683: f64, t737: f64, t754: f64, t2360: f64, t761: f64, t2344: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9735 = t89 * t9733 * t670;
    let t9744 = t2404 * t675;
    let t9770 = t683 * t2371;
    let t9787 = t737 * t754;
    let t9791 = t761 * t2360;
    let t9802 = t2344 * t675;
    (t9735, t9744, t9770, t9787, t9791, t9802)
}
