//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 650/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk650(t28149: f64, t28193: f64, t28233: f64, t28281: f64, t28325: f64, t28372: f64, t28413: f64, t28458: f64, t24191: f64, t6752: f64, t193: f64, t375: f64, t7087: f64, t89: f64) -> (f64, f64, f64) {
    let t28461 = t28149 + t28193 + t28233 + t28281 + t28325 + t28372 + t28413 + t28458;
    let t28466 = t24191 * t6752;
    let t28467 = t193 * t28466;
    let t28491 = t89 * t375 * t7087;
    (t28461, t28467, t28491)
}
