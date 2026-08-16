//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 975/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk975(t143332: f64, t1636: f64, t7658: f64, t89: f64, t33988: f64, t375: f64, t33860: f64, t6308: f64, t681: f64, t1486: f64, t2399: f64, t7650: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t143333 = 4.0_f64 / 27.0_f64 * t143332;
    let t143335 = t89 * t1636 * t7658;
    let t143336 = 4.0_f64 / 27.0_f64 * t143335;
    let t143339 = t89 * t375 * t33988;
    let t143355 = t6308 * t681 * t33860;
    let t143365 = t1486 * t2399 * t7650;
    (t143333, t143335, t143336, t143339, t143355, t143365)
}
