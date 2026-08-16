//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 941/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk941(t136516: f64, t78: f64, t32300: f64, t409: f64, t173: f64, t22557: f64, t32273: f64, t7195: f64, t32250: f64, t92335: f64, t1613: f64, t92354: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t136517 = t136516 * t78;
    let t136520 = t32300 * t409;
    let t136531 = t22557 * t7195 * t173 * t32273;
    let t136555 = t92335 * t32250;
    let t136558 = t1613 * sigma0;
    let t136559 = t92354 * t136558;
    (t136517, t136520, t136531, t136555, t136559)
}
