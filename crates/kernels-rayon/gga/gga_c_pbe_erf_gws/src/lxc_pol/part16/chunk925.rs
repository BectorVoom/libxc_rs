//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 925/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk925(t4366: f64, t954: f64, t1413: f64, t1528: f64, t34: f64, t6952: f64, t1416: f64, t2485: f64, t478: f64, t532: f64, t2488: f64, t39: f64) -> (f64, f64, f64, f64, f64) {
    let t8090 = t4366 * t954;
    let t8091 = t8090 * t1413;
    let t8093 = t1528 * t34;
    let t8094 = t8093 * t6952;
    let t8096 = t2485 * t1416;
    let t8098 = t478 * t532;
    let t8100 = t2488 * t39;
    (t8091, t8094, t8096, t8098, t8100)
}
