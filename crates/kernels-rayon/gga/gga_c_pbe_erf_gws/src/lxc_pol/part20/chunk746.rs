//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 746/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk746(t404: f64, t4536: f64, t389: f64, t4510: f64, t1291: f64, t1: f64, t2057: f64, t793: f64, t2062: f64, t700: f64, t762: f64, t1597: f64) -> (f64, f64, f64, f64, f64) {
    let t4537 = t4536 * t404;
    let t4538 = t389 * t4537;
    let t4539 = 1.0_f64 * t4538;
    let t4540 = t4510 * t404;
    let t4541 = t1291 * t4540;
    let t4542 = 6.0_f64 * t4541;
    let t4544 = t793 * t2057 * t1;
    let t4545 = t4544 * t2062;
    let t4550 = 0.50257692321302641125e0_f64 * t762 * t700;
    let t4554 = t1597 * t700;
    (t4539, t4542, t4545, t4550, t4554)
}
