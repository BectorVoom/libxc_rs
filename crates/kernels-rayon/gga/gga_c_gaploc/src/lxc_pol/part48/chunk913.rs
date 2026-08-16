//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 913/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk913(t13635: f64, t23157: f64, t11844: f64, t2021: f64, t7372: f64, t2976: f64, t44787: f64, t900: f64, t13625: f64, t22665: f64, t7427: f64, t2536: f64, t3601: f64) -> (f64, f64, f64, f64, f64) {
    let t45513 = t23157 * t13635;
    let t45516 = t2021 * t11844 * t7372;
    let t45517 = 0.14896037479937677779e-1_f64 * t45516;
    let t45519 = t2976 * t900 * t44787;
    let t45520 = 0.29792074959875355558e-1_f64 * t45519;
    let t45522 = t7427 * t22665 * t13625;
    let t45524 = t2536 * t3601;
    (t45513, t45517, t45520, t45522, t45524)
}
