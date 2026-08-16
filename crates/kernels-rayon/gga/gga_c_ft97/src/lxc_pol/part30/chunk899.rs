//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 899/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk899(t235: f64, t9680: f64, t683: f64, t7514: f64, t191: f64, t33300: f64, t2371: f64, t2404: f64, t190: f64, t251: f64, t36452: f64, t37991: f64) -> (f64, f64, f64, f64, f64) {
    let t41547 = 1.0_f64 / t9680 / t235;
    let t41825 = t683 * t7514;
    let t41848 = t191 * t33300;
    let t41879 = t2404 * t2371;
    let t42050 = 1.0_f64 / t251 / t37991 / t190 / t2371 / t36452 / 96.0_f64;
    (t41547, t41825, t41848, t41879, t42050)
}
