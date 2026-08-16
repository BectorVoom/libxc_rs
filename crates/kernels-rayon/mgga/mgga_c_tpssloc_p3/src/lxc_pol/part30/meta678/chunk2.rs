//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2122/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2122(t1458: f64, t19534: f64, t22461: f64, t24999: f64, t26103: f64, t33085: f64, t4072: f64, t5493: f64, t6517: f64, t671: f64, t90400: f64, t96361: f64, t96685: f64, t96686: f64, t96704: f64, t96706: f64, t96708: f64, t96711: f64, t96731: f64) -> f64 {
    let t96732 = 4.0_f64 * t1458 * t90400 + 4.0_f64 * t1458 * t96361 + 2.0_f64 * t19534 * t6517 + 2.0_f64 * t22461 * t5493 + 4.0_f64 * t24999 * t4072 + 2.0_f64 * t26103 * t5493 + 4.0_f64 * t33085 * t4072 + 2.0_f64 * t671 * t96686 + t96685 + t96704 + t96706 + t96708 + t96711 + t96731;
    t96732
}
