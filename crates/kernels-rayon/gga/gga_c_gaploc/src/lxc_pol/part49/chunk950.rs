//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 950/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk950(t42349: f64, t1445: f64, t26809: f64, t3085: f64, t4527: f64, t8411: f64, t9327: f64, t10556: f64, t1415: f64, t9321: f64, t2487: f64, t41878: f64, t6711: f64) -> (f64, f64, f64, f64, f64) {
    let t42350 = 0.11502877786176224903e1_f64 * t42349;
    let t42354 = 0.27606906686822939767e2_f64 * t4527 * t1445 * t26809 * t3085;
    let t42356 = 0.10725146985555128001e1_f64 * t8411 * t9327;
    let t42359 = 0.42900587942220512003e1_f64 * t1415 * t10556 * t9321;
    let t42363 = t2487 * t6711 * t41878;
    (t42350, t42354, t42356, t42359, t42363)
}
