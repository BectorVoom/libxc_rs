//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 994/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk994<F: Float>(t11239: F, t3143: F, t342: F, t12051: F, t3154: F, t12048: F, t1071: F, t3151: F, t3304: F, t3318: F, t11687: F, t4998: F) -> (F, F, F, F, F) {
    let t12077 = t11239 * t3143;
    let t12078 = t342 * t12077;
    let t12079 = t12051 * t3154;
    let t12080 = t12048 * t12079;
    let t12085 = t1071 * t3151;
    let t12086 = t12085 * t3304;
    let t12089 = t12085 * t3318;
    let t12094 = t11687 * t4998;
    (t12078, t12080, t12086, t12089, t12094)
}
