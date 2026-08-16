//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 106/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk106(t200: f64, t11: f64, rho1: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t202 = 0.942486901e0_f64 + 0.349064173e0_f64 * t200;
    let t203 = t202 * t202;
    let t204 = t11 * sigma2;
    let t205 = rho1 * rho1;
    let t206 = pow_1_3(rho1);
    let t207 = t206 * t206;
    let t209 = 1.0_f64 / t207 / t205;
    (t202, t203, t204, t205, t206, t209)
}
