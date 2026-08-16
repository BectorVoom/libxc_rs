//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1281/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1281(t30424: f64, t574: f64, t1849: f64, t8230: f64, t2180: f64, t6287: f64, t1774: f64, t510: f64, t6468: f64, t1268: f64, t19451: f64, t2181: f64, t2183: f64, t28002: f64, t28007: f64, t28030: f64, t4028: f64, t652: f64, t7458: f64, t7676: f64, t8221: f64, t8231: f64, t8235: f64, t8237: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30425 = t30424 * t574;
    let t30428 = t8230 * t1849;
    let t30433 = t6287 * t2180;
    let t30444 = t1774 * t8230;
    let t30447 = t510 * t30424;
    let t30454 = t2180 * t6468;
    let t30465 = 2.0_f64 * t1268 * t30425 + 4.0_f64 * t1268 * t30428 + 2.0_f64 * t1268 * t30454 - 2.0_f64 * t19451 * t2181 + 2.0_f64 * t19451 * t2183 - 4.0_f64 * t2181 * t28002 - 2.0_f64 * t2181 * t28030 + 4.0_f64 * t2183 * t28002 + 2.0_f64 * t2183 * t28007 - 2.0_f64 * t30433 * t652 - 4.0_f64 * t30444 * t652 - 2.0_f64 * t30447 * t652 - 4.0_f64 * t4028 * t8221 - 4.0_f64 * t4028 * t8231 + 4.0_f64 * t4028 * t8235 + 4.0_f64 * t4028 * t8237 - 4.0_f64 * t7458 * t8221 - 4.0_f64 * t7458 * t8231 + 4.0_f64 * t7676 * t8235 + 4.0_f64 * t7676 * t8237;
    (t30425, t30428, t30433, t30444, t30447, t30454, t30465)
}
