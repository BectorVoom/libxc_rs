//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 585/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk585(t7940: f64, t2265: f64, t942: f64, t2416: f64, t7487: f64, t2160: f64, t2339: f64, t638: f64, t2323: f64, t1540: f64, t511: f64, t650: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8304 = 0.39726959900411316772e-4_f64 * t7940;
    let t8310 = t942 * t2265;
    let t8311 = 0.4726e1_f64 * t8310;
    let t8328 = t7487 * t2416;
    let t8331 = t638 * t2160 * t2339;
    let t8334 = t638 * t2160 * t2323;
    let t8339 = t1540 * t511;
    let t8340 = t8339 * t650;
    (t8304, t8311, t8328, t8331, t8334, t8339, t8340)
}
