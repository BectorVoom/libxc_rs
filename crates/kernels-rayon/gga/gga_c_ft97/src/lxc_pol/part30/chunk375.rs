//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 375/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk375(t6222: f64, t6223: f64, t193: f64, t1701: f64, t6027: f64, t811: f64, t820: f64, t6: f64, t816: f64, t8: f64, t2691: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6224 = t6222 * t6223;
    let t6225 = t193 * t6224;
    let t6229 = t1701 * t6027 * t811;
    let t6233 = t1701 * t6027 * t820;
    let t6240 = t816 * t6;
    let t6241 = t6240 * t8;
    let t6242 = t2691 * t6241;
    (t6224, t6225, t6229, t6233, t6240, t6241, t6242)
}
