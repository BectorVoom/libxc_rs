//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1189/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1189(t36273: f64, t4680: f64, t7575: f64, t8609: f64, t7564: f64, t8613: f64, t1181: f64, t4718: f64, t604: f64, t7426: f64, t31349: f64, t3360: f64, t4284: f64) -> (f64, f64, f64, f64, f64) {
    let t36274 = 0.10718504529517434243e-2_f64 * t36273;
    let t36276 = t7575 * t4680 * t8609;
    let t36279 = t7564 * t4680 * t8613;
    let t36283 = t7426 * t1181 * t604 * t4718;
    let t36284 = 0.42874018118069736972e-3_f64 * t36283;
    let t36286 = t3360 * t31349 * t4284;
    (t36274, t36276, t36279, t36284, t36286)
}
