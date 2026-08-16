//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 747/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk747(t28: f64, t265: f64, t504: f64, t4324: f64, t5098: f64, t1081: f64, t1260: f64, t1409: f64, t1534: f64, t1649: f64, t1768: f64, t3966: f64, t4332: f64, t506: f64, t52: f64, t607: f64, t873: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t5099 = piecewise3(t505, t5098, t4324);
    let t5106 = piecewise3(t401, t4324 * t28 / 2.0_f64 + t1534 * t1081 / 2.0_f64 + t873 * t1649 / 2.0_f64 - t4332, -t1260 * t1409 / 2.0_f64 - t1768 * t607 / 2.0_f64 - t506 * t3966 / 2.0_f64 + t5099 * t52 / 2.0_f64);
    (t5099, t5106)
}
