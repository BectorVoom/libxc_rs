//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 576/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk576(t140: f64, t4699: f64, t527: f64, t1013: f64, t2058: f64, t133: f64, t2066: f64, t3086: f64, t4481: f64, t4485: f64, t4489: f64, t550: f64, t2001: f64, t4675: f64, t4677: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t141 = 0.1e-59_f64 < t140;
    let t4700 = t527 * t4699;
    let t4702 = t1013 * t1013;
    let t4703 = t2058 * t4702;
    let t4704 = t133 * t4703;
    let t4710 = -t2066 + 0.11113000182098765433e-1_f64 * t3086 + 0.22226000364197530865e-1_f64 * t4481 - 0.33339000546296296298e-1_f64 * t4485 + 0.16669500273148148149e-1_f64 * t4489;
    let t4711 = t550 * t4710;
    let t4712 = t133 * t4711;
    let t4714 = piecewise3(t141, -4.0_f64 * t2001 * t4677 + 2.0_f64 * t4675 + 2.0_f64 * t4700 + 2.0_f64 * t4704 - t4712, 0.0_f64);
    (t4700, t4702, t4703, t4704, t4710, t4711, t4712, t4714)
}
