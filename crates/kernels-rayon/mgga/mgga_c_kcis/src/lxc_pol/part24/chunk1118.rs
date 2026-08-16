//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1118/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1118(t11229: f64, t1278: f64, t3621: f64, t11181: f64, t413: f64, t429: f64, t11182: f64, t1236: f64, t11228: f64, t433: f64, t436: f64, t782: f64, t9266: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34650 = t1278 * t11229;
    let t34689 = t3621 * t3621;
    let t34690 = 1.0_f64 / t34689;
    let t34814 = t413 / t11181 / t429;
    let t35547 = t1236 * t11182;
    let t35615 = t433 / t11228 / t436;
    let t35630 = t9266 * t782;
    (t34650, t34690, t34814, t35547, t35615, t35630)
}
