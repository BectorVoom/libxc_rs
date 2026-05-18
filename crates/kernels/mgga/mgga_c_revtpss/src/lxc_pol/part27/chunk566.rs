//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 566/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk566<F: Float>(t3153: F, t3302: F, t3154: F, t3300: F, t1043: F, t1071: F, t1089: F, t3133: F, t378: F, t1035: F, t3140: F, t342: F) -> (F, F, F, F, F, F, F) {
    let t3303 = t3153 * t3302;
    let t3304 = t3303 * t3154;
    let t3305 = t3300 * t3304;
    let t3309 = t1071 * t1043 * t1089;
    let t3313 = t378 * t3133 * t1089;
    let t3316 = t3140 * t1035;
    let t3317 = t342 * t3316;
    (t3303, t3304, t3305, t3309, t3313, t3316, t3317)
}
