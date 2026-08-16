//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1115/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1115(t1426: f64, t545: f64, t2022: f64, t7282: f64, t10073: f64, t2453: f64, t7283: f64, t136: f64, t2029: f64, t2457: f64, t2470: f64, t7285: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25937 = t1426 * t545;
    let t25938 = t25937 * t2022;
    let t25939 = t7282 * t25938;
    let t25941 = 0.24093411633903331839e-3_f64 * t10073 * t25939;
    let t25944 = t2453 * t7283;
    let t25945 = t2029 * t136;
    let t25946 = t25945 * t2457;
    let t25948 = 0.17135234354032049604e-2_f64 * t25944 * t25946;
    let t25953 = t7285 * t2470;
    (t25937, t25938, t25939, t25941, t25944, t25945, t25946, t25948, t25953)
}
