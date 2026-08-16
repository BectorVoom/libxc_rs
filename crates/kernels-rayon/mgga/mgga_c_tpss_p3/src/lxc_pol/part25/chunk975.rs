//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 975/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk975(t13358: f64, t13446: f64, t10289: f64, t10292: f64, t1317: f64, t13296: f64, t13298: f64, t13309: f64, t13312: f64, t13317: f64, t1976: f64, t1981: f64, t3418: f64, t3423: f64, t3486: f64, t4570: f64, t4626: f64, t578: f64, t619: f64, t7682: f64, t7690: f64, t91: f64) -> (f64, f64) {
    let t13447 = t13358 + t13446;
    let t13450 = -8.0_f64 * t10289 * t1317 + 40.0_f64 * t10292 * t3423 + t13296 * t91 - 4.0_f64 * t13298 * t619 - 120.0_f64 * t13309 * t7690 + 40.0_f64 * t13312 * t1981 + 20.0_f64 * t13317 * t1981 - 4.0_f64 * t13447 * t578 - 4.0_f64 * t1976 * t4626 - 8.0_f64 * t3418 * t3486 + 20.0_f64 * t4570 * t7682;
    (t13447, t13450)
}
