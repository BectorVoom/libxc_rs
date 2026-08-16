//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1126/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1126(t15488: f64, t3052: f64, t357: f64, t5229: f64, t339: f64, t454: f64, t242: f64, t3090: f64, t5068: f64, t1125: f64, t5072: f64, t1120: f64, t5231: f64) -> (f64, f64, f64, f64, f64) {
    let t15489 = t3052 * t15488;
    let t15491 = t5229 * t357;
    let t15493 = t339 * t454 * t15491;
    let t15499 = t242 * t3090 * t5068;
    let t15500 = t1125 * t15499;
    let t15503 = t242 * t3090 * t5072;
    let t15504 = t1125 * t15503;
    let t15506 = t5231 * t1120;
    (t15489, t15493, t15500, t15504, t15506)
}
