//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1005/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1005(t3229: f64, t797: f64, t1103: f64, t3128: f64, t1053: f64, t1102: f64, t3162: f64, t3446: f64, t3453: f64, t3165: f64, t2201: f64, t3602: f64, t3613: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12428 = t797 * t3229;
    let t12435 = t1103 * t3128;
    let t12437 = t1102 * t1053 * t12435;
    let t12440 = t3446 * t3453 * t3162;
    let t12443 = t3446 * t3453 * t3165;
    let t12446 = t2201 * t3613 * t3602;
    (t12428, t12435, t12437, t12440, t12443, t12446)
}
