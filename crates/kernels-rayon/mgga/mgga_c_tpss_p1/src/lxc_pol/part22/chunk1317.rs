//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1317/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1317(t2161: f64, t35764: f64, t10584: f64, t2364: f64, t1395: f64, t226: f64, t3721: f64, t782: f64, t36075: f64, t10667: f64, t19671: f64, t30: f64, t31814: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t64164 = t35764 * t2161;
    let t64168 = t10584 * t2161;
    let t64183 = t10584 * t2364;
    let t64190 = t1395 * t2364 * t226;
    let t64198 = t3721 * t782 * t226;
    let t64204 = t36075 * t226;
    let t64241 = t19671 * t10667;
    let t64247 = t31814 * t30;
    (t64164, t64168, t64183, t64190, t64198, t64204, t64241, t64247)
}
