//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2521/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2521(t10538: f64, t51297: f64, t213: f64, t225: f64, t40321: f64, t14574: f64, t2439: f64, t2777: f64, t10069: f64, t14504: f64, t14557: f64, t9303: f64) -> (f64, f64, f64, f64, f64) {
    let t51298 = t51297 * t10538;
    let t51299 = 0.34697458558045176417e-2_f64 * t51298;
    let t51320 = t213 * t225 * t40321;
    let t51355 = t2439 * t2777 * t14574;
    let t51373 = t10069 * t14504;
    let t51374 = 0.21951497276451705329e-1_f64 * t51373;
    let t51390 = t9303 * t14557;
    (t51299, t51320, t51355, t51374, t51390)
}
