//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 965/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk965(t1563: f64, t797: f64, t10997: f64, t3275: f64, t113: f64, t1561: f64) -> (f64, f64, f64) {
    let t10998 = t797 * t1563;
    let t11000 = t3275 * t10997 * t10998;
    let t11001 = 45.0_f64 / 64.0_f64 * t11000;
    let t11002 = t113 * t1561;
    (t10998, t11001, t11002)
}
