//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1286/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1286(t14571: f64, t32616: f64, t20019: f64, t25198: f64, t7069: f64, t29074: f64, t29078: f64, t23104: f64, t3005: f64, t7396: f64, t20671: f64, t24505: f64, t28069: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33929 = 0.15889106645266856297e0_f64 * t14571 * t32616;
    let t33932 = 0.23833659967900284446e0_f64 * t25198 * t20019 * t7069;
    let t33933 = 0.31952438294933958064e-1_f64 * t29074;
    let t33934 = 0.31952438294933958064e-1_f64 * t29078;
    let t33936 = t23104 * t3005 * t7396;
    let t33937 = 0.38342925953920749676e0_f64 * t33936;
    let t33942 = t28069 * t20671 * t24505;
    (t33929, t33932, t33933, t33934, t33937, t33942)
}
