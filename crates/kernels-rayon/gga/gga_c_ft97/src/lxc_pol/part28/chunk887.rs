//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 887/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk887(t2185: f64, t35050: f64, t605: f64, t167: f64, t34822: f64, t3578: f64, t574: f64, t7357: f64, t33039: f64, t925: f64, t2221: f64, t27015: f64, t6708: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35052 = t2185 * t605 * t35050;
    let t35056 = t2185 * t167 * t34822;
    let t35060 = t574 * t3578 * t7357;
    let t35063 = t33039 * t925;
    let t35064 = t2221 * t35063;
    let t35067 = t27015 * t6708;
    (t35052, t35056, t35060, t35063, t35064, t35067)
}
