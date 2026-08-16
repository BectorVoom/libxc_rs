//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 771/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk771(t1071: f64, t1083: f64, t2844: f64, t1160: f64, t318: f64, t86: f64, t284: f64, t3473: f64, t3177: f64, t3436: f64, t1194: f64, t381: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10560 = t1083 * t1071;
    let t10583 = t1083 * t2844;
    let t10631 = t86 * t318 * t1160;
    let t10707 = t3473 * t284;
    let t10745 = t3177 * t3436;
    let t10752 = t381 * t1194;
    (t10560, t10583, t10631, t10707, t10745, t10752)
}
