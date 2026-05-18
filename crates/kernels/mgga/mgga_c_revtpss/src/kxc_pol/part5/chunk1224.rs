//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1224/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1224<F: Float>(t19466: F, t19476: F, t1089: F, t378: F, t3302: F, t357: F, t4866: F, t4893: F, t1071: F, t6299: F, t1043: F, t16560: F) -> (F, F, F, F, F, F) {
    let t19477 = t19466 + t19476;
    let t19479 = t378 * t19477 * t1089;
    let t19482 = t3302 * t357;
    let t19483 = t19482 * t4866;
    let t19484 = t4893 * t19483;
    let t19488 = t1071 * t6299 * t1089;
    let t19491 = t16560 * t1043;
    (t19477, t19479, t19482, t19484, t19488, t19491)
}
