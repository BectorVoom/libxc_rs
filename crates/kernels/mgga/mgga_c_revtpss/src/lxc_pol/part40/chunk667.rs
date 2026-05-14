//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 667/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk667<F: Float>(t1082: F, t3059: F, t1086: F, t378: F, t994: F, t1089: F, t3118: F, t1071: F, t359: F, t999: F, t3075: F, t3140: F, t3143: F) -> (F, F, F, F, F, F, F, F) {
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
