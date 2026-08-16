//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 928/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk928(t248: f64, t3570: f64, t6230: f64, t3515: f64, t1243: f64, t19045: f64, t225: f64, t6151: f64, t6153: f64, t6239: f64, t3640: f64, t6270: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19095 = t248 * t3570 * t6230;
    let t19096 = t3515 * t19095;
    let t19201 = t19045 * t1243;
    let t19232 = t6151 * t225;
    let t19234 = t6153 * t225;
    let t19249 = t6239 * t225;
    let t19267 = t6270 * t3640;
    (t19095, t19096, t19201, t19232, t19234, t19249, t19267)
}
