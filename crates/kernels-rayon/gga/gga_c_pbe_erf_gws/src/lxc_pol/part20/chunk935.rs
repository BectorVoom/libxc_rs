//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 935/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk935(t3455: f64, t582: f64, t185: f64, t2705: f64, t954: f64, t7194: f64, t1620: f64, t2570: f64, t34: f64, t2612: f64, t2685: f64, t2572: f64, t7527: f64) -> (f64, f64, f64, f64, f64) {
    let t10485 = t582 * t3455;
    let t10486 = t185 * t10485;
    let t10487 = 8.0_f64 / 45.0_f64 * t10486;
    let t10488 = t2705 * t954;
    let t10489 = t7194 * t10488;
    let t10491 = 16.0_f64 / 45.0_f64 * t1620 * t10489;
    let t10492 = t2570 * t34;
    let t10493 = t7194 * t10492;
    let t10495 = 32.0_f64 / 45.0_f64 * t1620 * t10493;
    let t10497 = 8.0_f64 / 45.0_f64 * t2612 * t2685;
    let t10499 = 16.0_f64 / 45.0_f64 * t7527 * t2572;
    (t10487, t10491, t10495, t10497, t10499)
}
