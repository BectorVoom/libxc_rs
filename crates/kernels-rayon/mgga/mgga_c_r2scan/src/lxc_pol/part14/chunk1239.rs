//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1239/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1239(t1013: f64, t1120: f64, t11217: f64, t11223: f64, t12256: f64, t12259: f64, t1292: f64, t1295: f64, t1300: f64, t19203: f64, t2394: f64, t2400: f64, t3506: f64, t3735: f64, t38783: f64, t38839: f64, t6693: f64, t829: f64, t8398: f64, t8409: f64, t8412: f64, t8415: f64) -> f64 {
    let t41854 = -0.768e1_f64 * t6693 * t12256 * t829 - 0.768e1_f64 * t6693 * t12259 * t829 - 0.384e1_f64 * t6693 * t3735 * t1292 - 0.1536e2_f64 * t19203 * t3735 * t1295 - 0.768e1_f64 * t38839 * t2400 - 0.768e1_f64 * t11223 * t8412 - 0.384e1_f64 * t11223 * t8415 - 0.1536e2_f64 * t38783 * t8409 - 0.128e1_f64 * t1300 * t11217 * t1013 - 0.256e1_f64 * t1300 * t3506 * t2394 - 0.128e1_f64 * t1300 * t1120 * t8398;
    t41854
}
