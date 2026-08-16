//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 649/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk649(t24789: f64, t3876: f64, t1901: f64, t193: f64, t24841: f64, t24843: f64, t28417: f64, t28422: f64, t28426: f64, t28430: f64, t28434: f64, t28438: f64, t28441: f64, t28445: f64, t28448: f64, t28451: f64, t28453: f64, t446: f64, t89: f64) -> f64 {
    let t28455 = t24789 * t3876;
    let t28458 = t24841 / 9.0_f64 + t24843 / 9.0_f64 + t89 * t193 * t28417 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t28422 + t446 * t28426 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t28430 + 2.0_f64 / 3.0_f64 * t446 * t28434 + 2.0_f64 / 3.0_f64 * t446 * t28438 + 2.0_f64 / 3.0_f64 * t446 * t28441 + t446 * t28445 / 3.0_f64 - t446 * t28448 / 3.0_f64 + t28451 / 9.0_f64 + t28453 / 9.0_f64 + t1901 * t28455 / 9.0_f64;
    t28458
}
