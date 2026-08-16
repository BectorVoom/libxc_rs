//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 760/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk760(t4205: f64, t708: f64, t1462: f64, t2427: f64, t2373: f64, t2377: f64, t2408: f64, t4097: f64, t4099: f64, t4100: f64, t4103: f64, t4198: f64, t4201: f64, t4204: f64) -> (f64, f64, f64) {
    let t4207 = 4.0_f64 * t4205 * t708;
    let t4209 = 4.0_f64 * t2427 * t1462;
    let t4210 = t4097 + t4099 + t4100 + t4103 + t4198 - t4201 + t2373 + t2377 + t4204 + t4207 + t4209 + t2408;
    (t4207, t4209, t4210)
}
