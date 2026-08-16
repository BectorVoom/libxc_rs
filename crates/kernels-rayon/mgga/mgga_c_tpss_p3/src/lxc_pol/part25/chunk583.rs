//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 583/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk583(t1215: f64, t3240: f64, t159: f64, t527: f64, t210: f64, t1218: f64, t521: f64) -> (f64, f64, f64, f64) {
    let t3241 = t3240 * t1215;
    let t3243 = t159 * t527;
    let t3244 = t210 * t3243;
    let t3255 = 1.0_f64 / t1218 / t521;
    (t3241, t3243, t3244, t3255)
}
