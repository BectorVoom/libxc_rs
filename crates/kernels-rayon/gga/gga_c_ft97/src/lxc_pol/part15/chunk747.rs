//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 747/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk747(t20972: f64, t605: f64, t144: f64, t13201: f64, t17432: f64, t17434: f64, t17436: f64, t17438: f64, t17440: f64, t17443: f64, t1901: f64, t20927: f64, t20931: f64, t20935: f64, t20939: f64, t20942: f64, t20945: f64, t446: f64, t9457: f64) -> (f64, f64, f64) {
    let t20973 = t605 * t20972;
    let t20974 = t144 * t20973;
    let t20977 = -2.0_f64 / 9.0_f64 * t17432 + 2.0_f64 / 27.0_f64 * t17434 + t17436 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t17438 + 2.0_f64 / 9.0_f64 * t17440 - t17443 / 3.0_f64 + t1901 * t20927 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t1901 * t20931 - 2.0_f64 / 3.0_f64 * t1901 * t20935 - t446 * t20939 - t446 * t20942 + 2.0_f64 * t446 * t20945 - 4.0_f64 / 9.0_f64 * t13201 - t446 * t20974 / 3.0_f64 - t9457;
    (t20973, t20974, t20977)
}
