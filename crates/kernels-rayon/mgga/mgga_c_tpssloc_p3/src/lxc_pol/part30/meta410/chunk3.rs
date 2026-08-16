//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1553/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1553(t18050: f64, t18168: f64, t1068: f64, t1070: f64, t17194: f64, t17197: f64, t17198: f64, t17202: f64, t17209: f64, t17301: f64, t17303: f64, t17306: f64, t17372: f64, t17374: f64, t17377: f64, t17379: f64, t17425: f64, t17427: f64, t17561: f64, t17563: f64, t17568: f64, t193: f64, t336: f64, t4696: f64, t4700: f64, t4701: f64) -> (f64, f64) {
    let t18169 = t18050 + t18168;
    let t18173 = t1070 * t18169 * t193 * t336 + 2.0_f64 * t1068 * t17198 * t4700 - t1068 * t17202 * t4700 - 2.0_f64 * t4696 * t4700 * t4701 + t17194 + t17197 - t17209 - t17301 - t17303 - t17306 + t17372 + t17374 - t17377 + t17379 + t17425 + t17427 + t17561 - t17563 - t17568;
    (t18169, t18173)
}
