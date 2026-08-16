//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 924/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk924(t4351: f64, t950: f64, t1403: f64, t1523: f64, t34: f64, t6937: f64, t1407: f64, t2477: f64, t476: f64, t532: f64, t2480: f64, t39: f64) -> (f64, f64, f64, f64, f64) {
    let t8078 = t4351 * t950;
    let t8079 = t8078 * t1403;
    let t8081 = t1523 * t34;
    let t8082 = t8081 * t6937;
    let t8084 = t2477 * t1407;
    let t8086 = t476 * t532;
    let t8088 = t2480 * t39;
    (t8079, t8082, t8084, t8086, t8088)
}
