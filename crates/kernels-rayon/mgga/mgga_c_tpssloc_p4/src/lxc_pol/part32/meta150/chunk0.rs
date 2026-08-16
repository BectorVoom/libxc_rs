//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 804/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk804(t40: f64, t52: f64, t185: f64, t4098: f64, t1472: f64, t751: f64, t1409: f64, t707: f64, t75: f64, t3966: f64, t607: f64, t767: f64, t78: f64, t771: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t4099 = t4098 * t185;
    let t4100 = t1472 * t751;
    let t4101 = t751 * t1409;
    let t4102 = t707 * t4101;
    let t4103 = 4.0_f64 * t4102;
    let t4104 = t75 * t1409;
    let t4110 = piecewise3(t146, 0.0_f64, -2.0_f64 / 9.0_f64 * t4104 * t607 + 2.0_f64 / 3.0_f64 * t767 * t3966);
    let t4111 = t78 * t1409;
    let t4117 = piecewise3(t150, 0.0_f64, -2.0_f64 / 9.0_f64 * t4111 * t607 - 2.0_f64 / 3.0_f64 * t771 * t3966);
    (t4099, t4100, t4101, t4102, t4103, t4104, t4110, t4111, t4117)
}
