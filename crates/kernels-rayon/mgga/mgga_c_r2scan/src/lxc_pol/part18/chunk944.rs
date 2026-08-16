//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 944/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk944(t10950: f64, t3434: f64, t3437: f64, t2317: f64, t502: f64, t3446: f64, t3448: f64, t10949: f64, t874: f64, t3447: f64, t2312: f64, t3438: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10952 = t3434 * t3437 * t10950;
    let t10954 = t502 * t2317;
    let t10956 = t3446 * t10954 * t3448;
    let t10957 = 0.81300399444200075504e-3_f64 * t10956;
    let t10958 = t10949 * t874;
    let t10960 = t3446 * t3447 * t10958;
    let t10962 = t3438 * t2312;
    (t10952, t10954, t10957, t10958, t10960, t10962)
}
