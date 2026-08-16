//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 959/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk959(t10946: f64, t3429: f64, t158: f64, t607: f64, t122: f64, t3434: f64, t3437: f64, t2317: f64, t502: f64, t3446: f64, t3448: f64, t874: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10947 = t3429 * t10946;
    let t10948 = 0.81300399444200075504e-3_f64 * t10947;
    let t10949 = t158 * t607;
    let t10950 = t10949 * t122;
    let t10952 = t3434 * t3437 * t10950;
    let t10954 = t502 * t2317;
    let t10956 = t3446 * t10954 * t3448;
    let t10957 = 0.81300399444200075504e-3_f64 * t10956;
    let t10958 = t10949 * t874;
    (t10948, t10949, t10950, t10952, t10954, t10957, t10958)
}
