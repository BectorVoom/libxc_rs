//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2571/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2571(t43859: f64, t44249: f64, t44275: f64, t51299: f64, t51310: f64, t71203: f64, t71206: f64, t71499: f64, t71501: f64, t71505: f64, t71508: f64, t71511: f64) -> f64 {
    let t71978 = 0.309885e1_f64 * t71203 + 0.929655e1_f64 * t71206 + t44249 + 0.794188125e1_f64 * t71499 - 0.473371875e0_f64 * t71501 - 0.30872592592592592592e0_f64 * t43859 - 0.62517e0_f64 * t71505 + 0.187551e1_f64 * t71508 + 0.13892666666666666667e0_f64 * t71511 - t51299 + t51310 + t44275;
    t71978
}
