//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1287/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1287(t11039: f64, t2194: f64, t1445: f64, t2530: f64, t813: f64, t8528: f64, t2949: f64, t7112: f64, t3492: f64, t6024: f64, t16239: f64, t3477: f64) -> (f64, f64, f64, f64, f64) {
    let t33912 = 0.92023022289409799224e1_f64 * t2194 * t11039;
    let t33916 = 0.92023022289409799224e1_f64 * t813 * t1445 * t8528 * t2530;
    let t33920 = 0.46011511144704899612e1_f64 * t813 * t1445 * t2949 * t7112;
    let t33922 = 0.11502877786176224903e2_f64 * t6024 * t3492;
    let t33927 = 0.71500979903700853338e0_f64 * t16239 * t3477;
    (t33912, t33916, t33920, t33922, t33927)
}
