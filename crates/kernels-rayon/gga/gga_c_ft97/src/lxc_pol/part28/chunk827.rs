//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 827/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk827(t160: f64, t7312: f64, t379: f64, t9133: f64, t23443: f64, t5856: f64, t23571: f64, t5947: f64, t12968: f64, t27015: f64, t5956: f64, t13140: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33191 = t160 * t7312;
    let t33192 = t33191 * t379;
    let t33193 = t9133 * t33192;
    let t33196 = t23443 * t5856;
    let t33199 = t23571 * t5947;
    let t33200 = t12968 * t33199;
    let t33203 = t27015 * t5956;
    let t33204 = t13140 * t33203;
    (t33191, t33192, t33193, t33196, t33199, t33200, t33203, t33204)
}
