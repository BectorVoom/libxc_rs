//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 806/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk806(t1082: f64, t3059: f64, t1086: f64, t378: f64, t994: f64, t1089: f64, t3118: f64, t1071: f64, t359: f64, t999: f64, t3075: f64, t3140: f64, t3143: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3283 = t1082 * t3059;
    let t3286 = t1086 * t378;
    let t3287 = t994 * t3286;
    let t3288 = t3118 * t1089;
    let t3291 = t359 * t1071;
    let t3292 = t3291 * t999;
    let t3295 = t1082 * t3075;
    let t3298 = t3140 * t3143;
    (t3283, t3286, t3287, t3288, t3291, t3292, t3295, t3298)
}
