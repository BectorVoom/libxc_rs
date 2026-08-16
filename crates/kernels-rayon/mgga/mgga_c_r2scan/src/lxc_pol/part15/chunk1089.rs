//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1089/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1089(t38228: f64, t10930: f64, t158: f64, t2304: f64, t2317: f64, t3434: f64, t357: f64, t6854: f64, t862: f64, t1615: f64, t875: f64, t269: f64, t3438: f64) -> (f64, f64, f64, f64) {
    let t38229 = 0.64980365807044550255e-5_f64 * t38228;
    let t38233 = t3434 * t2304 * t2317 * t158 * t10930;
    let t38234 = 0.5854811038705731867e-3_f64 * t38233;
    let t38240 = t862 * t357 * t6854;
    let t38241 = t1615 * t875;
    let t38244 = t38240 * t38241 * t3438 * t269;
    (t38229, t38234, t38241, t38244)
}
