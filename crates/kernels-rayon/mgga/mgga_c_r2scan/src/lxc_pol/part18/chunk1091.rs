//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1091/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1091(t39395: f64, t10772: f64, t10810: f64, t2578: f64, t1577: f64, t2599: f64, t2096: f64, t2649: f64, t571: f64, t10769: f64, t2547: f64, t37764: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39396 = 0.12805040077930161442e0_f64 * t39395;
    let t39400 = t10772 * t10810 * t2578;
    let t39401 = 0.69345773920434148506e0_f64 * t39400;
    let t39403 = t1577 * t10810 * t2599;
    let t39404 = 0.46230515946956099004e0_f64 * t39403;
    let t39409 = t571 * t2649 * t2096;
    let t39410 = t39409 * t10769;
    let t39411 = 0.47609969197673950972e-2_f64 * t39410;
    let t39420 = t37764 * t2547;
    (t39396, t39401, t39404, t39409, t39411, t39420)
}
