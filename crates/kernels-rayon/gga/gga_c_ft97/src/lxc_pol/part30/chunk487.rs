//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 487/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk487(t7679: f64, t871: f64, t296: f64, t193: f64, t446: f64, t7622: f64, t7626: f64, t7631: f64, t7635: f64, t7664: f64, t7669: f64, t7674: f64, t89: f64) -> (f64, f64, f64) {
    let t7680 = t871 * t7679;
    let t7681 = t296 * t7680;
    let t7684 = 2.0_f64 / 3.0_f64 * t446 * t7622 - 2.0_f64 / 3.0_f64 * t446 * t7626 + 2.0_f64 / 3.0_f64 * t446 * t7631 - t446 * t7635 / 3.0_f64 + t89 * t193 * t7664 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t7669 + 2.0_f64 / 3.0_f64 * t446 * t7674 - t446 * t7681 / 3.0_f64;
    (t7680, t7681, t7684)
}
