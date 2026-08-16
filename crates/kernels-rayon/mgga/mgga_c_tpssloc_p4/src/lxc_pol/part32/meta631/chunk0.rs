//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2043/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2043(t87291: f64, t23062: f64, t25106: f64, t13176: f64, t6613: f64, t23133: f64, t4257: f64, t1496: f64, t81942: f64, t7497: f64, t81933: f64, t25098: f64, t81835: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t87292 = 7.0_f64 / 72.0_f64 * t87291;
    let t87293 = t23062 * t25106;
    let t87295 = t13176 * t6613;
    let t87300 = t23133 * t4257;
    let t87301 = 35.0_f64 / 288.0_f64 * t87300;
    let t87304 = t81942 * t1496;
    let t87306 = t81933 * t7497;
    let t87308 = t81835 * t25098;
    (t87292, t87293, t87295, t87301, t87304, t87306, t87308)
}
