//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 949/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk949(t1403: f64, t2336: f64, t33546: f64, t2252: f64, t342: f64, t7430: f64, t33561: f64, t630: f64, t24499: f64, t24220: f64, t7437: f64, t33583: f64, t681: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t141478 = t1403 * t2336 * t33546;
    let t141489 = t342 * t2252 * t7430 / 18.0_f64;
    let t141491 = t342 * t630 * t33561;
    let t141509 = t1403 * t24499;
    let t141524 = t7437 * t24220;
    let t141527 = t1403 * t681 * t33583;
    (t141478, t141489, t141491, t141509, t141524, t141527)
}
