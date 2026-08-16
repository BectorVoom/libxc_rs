//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 841/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk841(t193: f64, t35529: f64, t89: f64, t33476: f64, t992: f64, t2354: f64, t446: f64, t1131: f64, t33253: f64, t35516: f64, t676: f64, t27: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35530 = t193 * t35529;
    let t35531 = t89 * t35530;
    let t35533 = t33476 * t992;
    let t35534 = t2354 * t35533;
    let t35535 = t446 * t35534;
    let t35537 = t33253 * t1131;
    let t35538 = t193 * t35537;
    let t35539 = t89 * t35538;
    let t35541 = t676 * t35516;
    let t35543 = t89 * t27 * t35541;
    (t35531, t35534, t35535, t35537, t35539, t35541, t35543)
}
