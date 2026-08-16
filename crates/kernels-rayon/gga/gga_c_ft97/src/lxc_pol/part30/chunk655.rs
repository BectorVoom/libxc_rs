//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 655/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk655(t668: f64, t7021: f64, t505: f64, t2665: f64, t446: f64, t28491: f64, t28494: f64, t28499: f64, t28504: f64, t28509: f64, t28514: f64, t28518: f64, t28522: f64, t28526: f64, t28529: f64, t28531: f64) -> (f64, f64, f64) {
    let t28533 = t7021 * t668;
    let t28534 = t28533 * t505;
    let t28535 = t2665 * t28534;
    let t28536 = t446 * t28535;
    let t28538 = t28491 / 9.0_f64 - t28494 / 36.0_f64 + t28499 / 3.0_f64 + t28504 / 3.0_f64 + t28509 / 3.0_f64 + t28514 / 12.0_f64 - 2.0_f64 / 9.0_f64 * t28518 - 2.0_f64 / 9.0_f64 * t28522 + 2.0_f64 / 27.0_f64 * t28526 + t28529 / 18.0_f64 - t28531 / 27.0_f64 + t28536 / 9.0_f64;
    (t28534, t28536, t28538)
}
