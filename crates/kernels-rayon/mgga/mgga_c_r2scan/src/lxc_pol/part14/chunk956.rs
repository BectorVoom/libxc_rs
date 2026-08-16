//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 956/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk956(t2315: f64, t3438: f64, t10992: f64, t3446: f64, t1563: f64, t797: f64, t113: f64, t1561: f64) -> (f64, f64, f64, f64) {
    let t10993 = t3438 * t2315;
    let t10995 = t3446 * t10992 * t10993;
    let t10998 = t797 * t1563;
    let t11002 = t113 * t1561;
    (t10993, t10995, t10998, t11002)
}
