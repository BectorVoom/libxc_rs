//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1595/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1595(t12256: f64, t13026: f64, t1204: f64, t3140: f64, t3599: f64, t11239: f64, t460: f64, t1242: f64, t474: f64, t11243: f64, t479: f64, t3603: f64, t471: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13027 = t13026 * t12256;
    let t13032 = t1204 * t3140;
    let t13033 = t13032 * t3599;
    let t13036 = t460 * t11239;
    let t13037 = t1242 * t1242;
    let t13038 = 1.0_f64 / t13037;
    let t13039 = t13038 * t474;
    let t13040 = t479 * t11243;
    let t13041 = t13039 * t13040;
    let t13042 = t13036 * t13041;
    let t13045 = t3603 * t471;
    (t13027, t13032, t13033, t13036, t13037, t13038, t13040, t13041, t13042, t13045)
}
