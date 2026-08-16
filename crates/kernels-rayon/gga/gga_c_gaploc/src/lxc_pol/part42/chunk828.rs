//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 828/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk828(t1063: f64, t11254: f64, t2343: f64, t6519: f64, t42898: f64, t37275: f64, t921: f64, t2497: f64, t3553: f64, t4349: f64, t27232: f64, t3366: f64) -> (f64, f64, f64, f64, f64) {
    let t44662 = 0.56910013271352299198e-1_f64 * t1063 * t2343 * t11254 * t6519;
    let t44665 = 0.47425011059460249332e-2_f64 * t42898;
    let t44671 = t37275 * t921;
    let t44674 = 6.0_f64 * t4349 * t3553 * t2497;
    let t44676 = 4.0_f64 * t27232 * t3366;
    (t44662, t44665, t44671, t44674, t44676)
}
