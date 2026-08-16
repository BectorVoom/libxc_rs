//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1284/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1284(t30534: f64, t574: f64, t1849: f64, t8273: f64, t1774: f64, t2199: f64, t6287: f64, t6468: f64, t510: f64, t1268: f64, t19451: f64, t2200: f64, t2202: f64, t28002: f64, t28007: f64, t28030: f64, t4028: f64, t652: f64, t7458: f64, t7676: f64, t8260: f64, t8274: f64, t8278: f64, t8280: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30535 = t30534 * t574;
    let t30538 = t8273 * t1849;
    let t30543 = t1774 * t8273;
    let t30558 = t6287 * t2199;
    let t30565 = t2199 * t6468;
    let t30574 = t510 * t30534;
    let t30581 = 2.0_f64 * t1268 * t30535 + 4.0_f64 * t1268 * t30538 + 2.0_f64 * t1268 * t30565 - 2.0_f64 * t19451 * t2200 + 2.0_f64 * t19451 * t2202 - 4.0_f64 * t2200 * t28002 - 2.0_f64 * t2200 * t28030 + 4.0_f64 * t2202 * t28002 + 2.0_f64 * t2202 * t28007 - 4.0_f64 * t30543 * t652 - 2.0_f64 * t30558 * t652 - 2.0_f64 * t30574 * t652 - 4.0_f64 * t4028 * t8260 - 4.0_f64 * t4028 * t8274 + 4.0_f64 * t4028 * t8278 + 4.0_f64 * t4028 * t8280 - 4.0_f64 * t7458 * t8260 - 4.0_f64 * t7458 * t8274 + 4.0_f64 * t7676 * t8278 + 4.0_f64 * t7676 * t8280;
    (t30535, t30538, t30543, t30558, t30565, t30574, t30581)
}
