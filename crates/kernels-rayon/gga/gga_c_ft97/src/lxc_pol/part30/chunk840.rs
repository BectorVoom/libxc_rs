//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 840/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk840(t35516: f64, t743: f64, t1434: f64, t193: f64, t33460: f64, t9770: f64, t992: f64, t446: f64, t1131: f64, t33243: f64, t89: f64, t6008: f64, t6837: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35517 = t743 * t35516;
    let t35519 = t1434 * t193 * t35517;
    let t35522 = t9770 * t33460 * t992;
    let t35523 = t446 * t35522;
    let t35525 = t33243 * t1131;
    let t35526 = t193 * t35525;
    let t35527 = t89 * t35526;
    let t35529 = t6008 * t6837;
    (t35517, t35519, t35522, t35523, t35525, t35527, t35529)
}
